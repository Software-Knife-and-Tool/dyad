;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; [*closure descriptor*] #s(:closure env nreqs rest func)
;;;
;;; env:   list of frame state structs
;;; nreqs: number of required arguments
;;; rest:  rest symbol or nil
;;; func:  function, mu applyable
;;;
(mu:intern core::ns :intern "closure-prop"
  (:lambda (key closure)
    (core:errorp-unless core:closurep closure "core::closure-prop: is not a closure")           
    (mu:sv-ref
     (mu:st-vec closure)
     (mu:cdr
      (core:assoc
       key
       '((:env   . 0)
         (:nreqs . 1)
         (:rest  . 2)
         (:func  . 3)))))))

(mu:intern core::ns :intern "make-closure"
  (:lambda (env nreq rest func)
    (mu:struct :closure
      (mu:cons env
        (mu:cons nreqs
          (mu:cons rest                
            (core::list func)))))))

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
     (:if (core::closurep fn)
          (:if (core::logand (core:closure-prop :rest fn) (core:zerop (core::closure-prop :arity fn)))
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
                   (core:dropr args (mu:fx-sub (mu:length args) (core::closure-prop :arity fn)))
                   (core:dropl args (core::closure-prop :arity fn))))
          (core::flat-arg-list args))))

(mu:intern core::ns :intern "quoted-lambda-arg-list"
   (:lambda (fn args)
     (:if (core::closure-prop :rest fn)
          (:if (core::logand (core:closure-prop :rest fn) (core:zerop (core::closure-prop :arity fn)))
               (core::list args)
               ((:lambda (reqs rest)
                  (core::append
                   reqs
                   (core::list rest)))
                (core:dropr args (mu:fx-sub (mu:length args) (core::closure-prop :arity fn)))
                (core:dropl args (core::closure-prop :arity fn))))
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

(mu:intern core::ns :intern "compile-quoted-lambda-arg-list"
   (:lambda (fn args env)
     (core::quoted-lambda-arg-list fn (core::compile-list args env))))

;;;
;;; core apply
;;;
(mu:intern core::ns :extern "apply"
   (:lambda (fn args)
     (core:errorp-unless
      (:lambda (fn) (core::logor (core:functionp fn) (core:closurep fn)))
      fn
      "core:apply: not a function or closure")
     (:if (core:functionp fn)
          (mu:apply fn args)
          (core::closure-apply fn args))))

(mu:intern core::ns :intern "closure-apply"
   (:lambda (fn args)
     ((:lambda (env)
        (core:mapc mu:fr-push env)
        ((:lambda (mu-fn)
           (:if (core::closure-prop :rest fn)
                (mu:apply mu-fn (core::quoted-lambda-arg-list fn args))
                (mu:apply mu-fn args)))
         (core::closure-prop :func fn))
        (core:mapc mu:fr-pop env))
      (core::closure-prop :env fn))))

;;;
;;; compile-application
;;;
;;; expand macros
;;; convert core lambdas to mu forms
;;; compile function applications
;;;
(mu:intern core::ns :intern "compile-lambda-call"
  (:lambda (form args env)
    ((:lambda (fn)
       (:if (core:closurep fn)
            (core::list3 core::apply-closure fn (core::compile-flat-arg-list args env))
            (mu:cons fn (core::compile-quoted-lambda-arg-list fn args env))))
     (core::compile form env))))

(mu:intern core::ns :intern "compile-fn-call"
  (:lambda (fn args env)
    (:if (core::closurep fn)
         (core::list3 core::apply-closure fn (core::compile-flat-arg-list args env))
         (mu:cons fn (core::compile-quoted-lambda-arg-list fn args env)))))

(mu:intern core::ns :intern "compile-macro-call"
  (:lambda (macro-symbol args env)
    (core::compile
     (core:macroexpand (mu:cons macro-symbol args) env)
     env)))

(mu:intern core::ns :intern "compile-symbol-call"
  (:lambda (symbol args env)
    (:if (core:boundp symbol)
         ((:lambda (fn)
            (:if (core::lambda-prop :desc fn)
                 (core::list3 core:apply fn (core::compile-flat-arg-list args env))
                 (mu:cons fn (core::compile-quoted-lambda-arg-list fn args env))))
          (core:symbol-value (core::compile symbol env)))
         (core::list3 core:apply (core::compile symbol env) (core::compile-flat-arg-list args env)))))

;;;
;;; compile function application
;;;
(mu:intern core::ns :intern "compile-application"
  (:lambda (fn args env)
    (:if (core:functionp fn)
         (core::compile-fn-call fn args env)
         (:if (core:consp fn)
              (core::compile-lambda-call fn args env)
              (:if (core:symbolp fn)
                   (:if (core:boundp fn)
                        ((:lambda (macro-fn)
                           (:if macro-fn
                                (core::compile-macro-call fn args env)
                                (core::symbol-call fn args env)))
                         (core:macro-function fn env))
                        (core::compile-symbol-call fn args env))
                   (core:error fn "compile-application: not a function designator"))))))
