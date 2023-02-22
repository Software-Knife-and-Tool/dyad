;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core lambdas
;;;

;;;
;;; [*lambda descriptor*] #s(:lambda lambda-syms req-syms rest-sym env closure)
;;;
;;; symbols:    list of symbols, suitable for :lambda
;;; reqsyms:    list of required symbols
;;; restsym:    rest symbol or nil
;;; env:        list of lambda descriptors
;;; closure:    list of closed symbol bindings
;;;
(mu:intern core::ns :intern "lambda-prop"
  (:lambda (key desc)
    (mu:sv-ref
     (mu:st-vec desc)
     (mu:cdr
      (core:assoc
       key
       '((:symbols . 0)
         (:reqsyms . 1)
         (:restsym . 2)
         (:env     . 3)
         (:closure . 4)))))))

(mu:intern core::ns :intern "make-lambda"
   (:lambda (lambda req-syms rest-sym env closure)
     (mu:struct :lambda
        (mu:cons lambda
          (mu:cons req-syms
            (mu:cons rest-sym
               (mu:cons env
                  (core::list closure))))))))

;;;
;;; core lambda form
;;;
;;; [*core lambda form*]
;;; (lambda ([symbol ...] [&rest rest-symbol]) . body)
;;;

;;;
;;; parse core lambda list
;;;
(mu:intern core::ns :intern "core-lambda"
   (:lambda (form env)
      (:if (core:findl (:lambda (el) (core:null (core:symbolp el))) form)
         (core:error form "core::core-lambda: list syntax")
         ((:lambda (desc)
            (:if (core:fixnump desc)
                 (core::make-lambda form form () env ())
                 desc))
          (core:foldl
            (:lambda (el acc)
               (:if (core:vectorp acc)
                  acc
                  (:if (mu:eq '&rest el)
                     (:if (mu:eq (mu:length form) (mu:fx-add 2 acc))
                          (core::make-lambda
                           (core::append (core:dropr form 2) (core:dropl form (core:1- (mu:length form))))
                           (core:dropr form 2)
                           (mu:nth (core:1- (mu:length form)) form)
                           env
                           ())
                          (core:error form "core::core-lambda rest botch"))
                     (core:1+ acc))))
            0
            form)))))

;;;
;;; compile a core lambda form
;;;
(mu:intern core::ns :intern "compile-lambda"
   (:lambda (form env)
     ((:lambda (lambda-desc body)
        (mu:compile
           (mu:cons :lambda
              (mu:cons (core::lambda-prop :symbols lambda-desc)
                       (core::compile-lambda-body lambda-desc body env)))))
      (core::core-lambda (mu:car form) env)
      (mu:nth 1 form))))

;;;
;;; compile a closure
;;;
(mu:intern core::ns :intern "closure"
   (:lambda (fn)
     ((:lambda (lambda-desc)
        (core:errorp-when core:null lambda-desc "core::closure: closing bare function")
        ((:lambda (desc body)
           (mu:compile
            (mu:cons :lambda
               (mu:cons
                (core::lambda-prop :symbols desc)
                (mu:cons desc (mu:cdr (core::fn-form fn)))))))
         (core::make-lambda
          (core::lambda-prop :symbols lambda-desc)
          (core::lambda-prop :reqsyms lambda-desc)
          (core::lambda-prop :restsym lambda-desc)
          (core::lambda-prop :env lambda-desc)
          (core:mapcar (:lambda (desc-) (mu:fr-get (core::lambda-prop :frame desc-)))
                       (core::lambda-prop :env lambda-desc)))
         (mu:cdr (core::fn-form fn))))
        (core::fn-lambda-desc fn))))

;;;
;;; [*closure-descriptor*] `(function . values)` 
;;;
(mu:intern core::ns :intern "compile-closure"
  (:lambda (form env)
    (core:maplist
     (:lambda (form-cons)
       ((:lambda (compiled-form)
          (:if (mu:cdr form-cons)
               compiled-form
               (:if (core:functionp compiled-form)
                    (core:compile (mu:cons 'core::closure (core::list compiled-form)))
                    compiled-form)))
        (core::compile (mu:car form-cons) env)))
     form)))

;;;
;;; `core::compile-lambda-body` resolves body symbols with `core::compile-symbol`, uses
;;; the runtime compiler for everything else, and returns a compiled body. If a body returns
;;; a function, a closure body is generated.
;;;
;;; `lambda-desc:` lambda descriptor
;;; `body:` list of body forms
;;; `env:` runtime lexical env
;;;
;;; `returns:` compiled body
;;;
(mu:intern core::ns :intern "compile-lambda-body"
  (:lambda (lambda-desc body env)
    (:if body
         ((:lambda (env)
              (core::compile-closure (mu:cons lambda-desc body) env))
            (core::compile-add-env lambda-desc env))
           ())))

;;;
;;; Compiling symbols
;;;
;;; `core::compile-symbol` looks up unqualified symbols in the current lexical environment. If found,
;;; it compiles a *frame-reference* from the *lexical-descriptor* returned by `core::symbol-frame`.
;;;
;;; [*lexical-descriptor*] `(lex-ref frame-id offset)`
;;;
;;; We have two sources of lexical bindings: from the *core* lambda being compiled and the
;;; current lexical environment.
;;;
;;; `symbol:` symbol to be compiled
;;;
;;; `env:` compiled environment
;;;
;;; `returns:` qualified symbol or *frame-ref*
;;;
(mu:intern core::ns :intern "compile-symbol"
   (:lambda (symbol env)
     ((:lambda (env-ref)
          (:if env-ref
               (core::compile
                (mu:cons 'mu::fr-ref
                   (mu:cons (mu:car env-ref)
                      (core::list (mu:cdr env-ref))))
                 env)
               (mu:compile symbol)))
       (core::symbol-frame symbol env))))

;;;
;;; Resolving body symbols
;;;
;;; *symbol-frame* looks up *symbol* in a compiled environment.
;;;
;;; if found return a *lexical-descriptor*, otherwise *nil*.
;;;
;;; [*lexical-descriptor*] `(frame-id . offset)`
;;;
;;; `symbol:` symbol to be compiled
;;;
;;; `env:` lexical compiled environment
;;;
;;; `returns:` *frame-ref* or *nil*
;;;
(mu:intern core::ns :intern "symbol-frame"
   (:lambda (symbol env)
     (core:foldl
      (:lambda (frame acc)
        (:if acc
             acc
             ((:lambda (offset)
                (:if offset
                     (mu:cons (core::lambda-prop :func frame) offset)
                     ()))
              (core:positionl (:lambda (el) (mu:eq el symbol)) (core::lambda-prop :symbols frame)))))
      ()
      env)))
