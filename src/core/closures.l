;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; closure struct
;;; #s(:closure env lambda nreqs rest func form)
;;;
(mu:intern core::ns :intern "make-closure"
  (:lambda (env nreq rest func)
    (mu:struct :closure
      (mu:cons env
        (mu:cons nreqs
          (mu:cons rest                
            (mu:cons func ())))))))

(mu:intern core::ns :intern "closure-prop"
  (:lambda (closure key)
    (mu:sv-ref
     (mu:cdr
      (core:assoc
       key
       '((:env . 0)
         (:nreqs . 1)
         (:rest . 2)
         (:func . 3))))
     (mu:st-vec closure))))

;;;
;;; core argument lists
;;;
(mu:intern core::ns :intern "flat-arg-list"
    (:lambda (args)
      (core:foldr
       (:lambda (elt acc)
         (mu:cons 'mu:cons (mu:cons elt (core::list acc))))
       ()
       args)))

(mu:intern core::ns :intern "lambda-arg-list"
   (:lambda (fn args)
     (:if (core::fn-restp fn)
          (:if (core:zerop (core::fn-nreqs fn))
               (core::list3 'mu:cons (core::flat-arg-list args) ())
               ((:lambda (reqs rest)
                  (mu:cons 'core::append
                     (mu:cons (core::flat-arg-list reqs)
                         (mu:cons               
                          (mu:cons 'mu:cons
                             (mu:cons        
                                (core::flat-arg-list rest)
                                (core::list ())))
                                        ()))))
                   (core:dropr args (mu:fx-sub (mu:length args) (core::fn-nreqs fn)))
                   (core:dropl args (core::fn-nreqs fn))))
          (core::flat-arg-list args))))

(mu:intern core::ns :intern "quoted-lambda-arg-list"
   (:lambda (fn args)
     (:if (core::fn-restp fn)
          (:if (core:zerop (core::fn-nreqs fn))
               (core::list args)
               ((:lambda (reqs rest)
                  (core::append
                   reqs
                   (core::list rest)))
                (core:dropr args (mu:fx-sub (mu:length args) (core::fn-nreqs fn)))
                (core:dropl args (core::fn-nreqs fn))))
          args)))

;;;
;;; compiled argument lists
;;;
(mu:intern core::ns :intern "compile-flat-arg-list"
    (:lambda (args env)
      (core:foldr
       (:lambda (elt acc)
         (mu:cons 'mu:cons (mu:cons (core::compile elt env) (core::list acc))))
       ()
       args)))

#|
(mu:intern core::ns :intern "compile-lambda-arg-list"
   (:lambda (fn args env)
     (:if (core::fn-restp fn)
          (:if (core:zerop (core::fn-nreqs fn))
               (core::list3 'mu:cons (core::compile-flat-arg-list args env) ())
               ((:lambda (reqs rest)
                  (mu:cons 'core::append
                     (mu:cons (core::compile-flat-arg-list reqs env)
                         (mu:cons               
                          (mu:cons 'mu:cons
                             (mu:cons        
                                (core::compile-flat-arg-list rest env)
                                (core::list ())))
                                        ()))))
                   (core:dropr args (mu:fx-sub (mu:length args) (core::fn-nreqs fn)))
                   (core:dropl args (core::fn-nreqs fn))))
          (core::compile-flat-arg-list args env))))
|#

(mu:intern core::ns :intern "compile-quoted-lambda-arg-list"
   (:lambda (fn args env)
     (core::quoted-lambda-arg-list fn (core::compile-list args env))))

;;;
;;; core apply
;;;
(mu:intern core::ns :intern "enter-env"
   (:lambda (fn)
     ((:lambda (lambda-desc)
        (:if lambda-desc
             (core:mapc
              mu:fr-push
              (core::lambda-closure lambda-desc))
             ()))
      (core::fn-lambda-desc fn))))

(mu:intern core::ns :intern "exit-env"
   (:lambda (fn)
     ((:lambda (lambda-desc)
        (:if lambda-desc
             (core:mapc
              (:lambda (frame)
                (mu:fr-pop (mu:car frame)))
              (core::lambda-closure lambda-desc))
             ()))
      (core::fn-lambda-desc fn))))

(mu:intern core::ns :intern "apply"
   (:lambda (fn args)
     (core::enter-env fn)
     (core::prog1
       (:if (core::fn-restp fn)
           (mu:apply fn (core::quoted-lambda-arg-list fn args))
           (mu:apply fn args))
       (core::exit-env fn))))

(mu:intern core::ns :extern "apply"
   (:lambda (fn args)
     (core:errorp-unless core:functionp fn "core:apply: not a function")
     (core:errorp-unless core:listp args "core:apply: not a list")
     (core::enter-env fn)
     (core::prog1
         (:if (core::fn-restp fn)
              (mu:apply fn (core::lambda-arg-list fn args))
              (mu:apply fn args))
       (core::exit-env fn))))

;;;
;;; compile-function-application
;;;
;;; expand macros
;;; convert core lambdas to mu forms
;;;
(mu:intern core::ns :intern "compile-function-application"
   (:lambda (fn args env)
     ((:lambda (cons-call fn-call macro-call symbol-call)
        (:if (core:functionp fn)
             (fn-call fn)
             (:if (core:consp fn)
                  (cons-call fn)
                  (:if (core:symbolp fn)
                       (:if (core:boundp fn)
                            ((:lambda (macro-fn)
                               (:if macro-fn
                                    (macro-call fn)
                                    (symbol-call fn)))
                             (core:macro-function fn env))
                            (symbol-call fn))
                       (core:error fn "compile-function-application: not a function designator")))))
      (:lambda (form)     ;;; lambda expression call
        ((:lambda (fn)
           (core:errorp-unless core:functionp fn "compile-function-application: lambda is not a function designator")
           (:if (core::fn-lambda-desc fn)
                (core::list3 core::apply fn (core::compile-flat-arg-list args env))
                (mu:cons fn (core::compile-quoted-lambda-arg-list fn args env))))
         (core::compile form env)))
      (:lambda (fn)        ;;; function call
        (:if (core::fn-lambda-desc fn)
             (core::list3 core::apply fn (core::compile-flat-arg-list args env))
             (mu:cons fn (core::compile-quoted-lambda-arg-list fn args env))))
      (:lambda (macro-symbol)  ;;; macro call
        (core::compile
         (core:macroexpand (mu:cons macro-symbol args) env)
         env))
      (:lambda (symbol)    ;;; symbol
        (:if (core:boundp symbol)
             ((:lambda (fn)
                (:if (core::fn-lambda-desc fn)
                     (core::list3 core::apply fn (core::compile-flat-arg-list args env))
                     (mu:cons fn (core::compile-quoted-lambda-arg-list fn args env))))
              (core:symbol-value (core::compile symbol env)))
             (core::list3 core::apply (core::compile symbol env) (core::compile-flat-arg-list args env)))))))
