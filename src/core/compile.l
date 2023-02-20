;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; compile.l: core compiler
;;;
;;; compiling an environment
;;; 
;;;    core::compile-add-env lambda-desc env => list
;;;
;;; core::complle-add-env adds a lambda descriptor to a list of
;;; lambda descriptors.
;;;
;;; arguments:
;;;
;;;   lambda-desc : lambda descriptor
;;;   env         : lexical environment list
;;;
;;; returns: augmented list of lambda descriptors
;;;
(mu:intern core::ns :intern "compile-add-env"
  (:lambda (lambda-desc env)
     (core:errorp-unless core:structp lambda-desc "compile-add-env: not a lambda descriptor")
     (core:errorp-unless core:listp env "compile-add-env: not a list") 
     (mu:cons lambda-desc env)))

;;;
;;; compiling special/macro forms
;;; 
;;;    core::defconst form env => list
;;;    core::defmacro form env => list
;;;    core::defun form env => list
;;;    core::if form env => list
;;;    core::lambda form env => list
;;;    core::quote form env => list
;;;
;;; special/macro form compiler rewrites a form against
;;; an environment into something the runtime can execute.
;;;
;;; arguments:
;;;
;;;   form : any form
;;;   env  : lexical environment list
;;;
;;; returns: mu form
;;;
(mu:intern core::ns :intern "quote" (:lambda (form env) form))

(mu:intern core::ns :intern "lambda"
    (:lambda (form env)
      (core:error-if (mu:fx-lt (mu:length form) 2) form "lambda: malformed lambda expression")
      (core::compile-lambda (core::list2 (mu:nth 1 form) (mu:nthcdr 2 form)) env)))

(mu:intern core::ns :intern "defconst"
   (:lambda (form env)
     ((:lambda (symbol value)
        (core:errorp-unless core:symbolp symbol "core::defconst: not a symbol")
        (core:errorp-when core:keywordp symbol "core::defconst: not a symbol")
        (mu:compile
         (core::list2 :quote
                      (core::intern
                       symbol
                       (mu:eval (core::compile value env))
                       env))))
      (mu:nth 1 form)
      (mu:nth 2 form))))

(mu:intern core::ns :intern "defmacro"
    (:lambda (form env)
      ((:lambda (symbol lambda)
         (core:errorp-unless core:symbolp symbol "core::defmacro: not a symbol")
         (core:errorp-when core:keywordp symbol "core::defmacro: not a symbol")
         (core:errorp-unless core:listp lambda "core::defmacro: not a lambda list")
         ((:lambda (macro-fn)
            ((:lambda (symbol)
               (core::core-macro-env symbol macro-fn)
               (mu:compile (core::list2 :quote symbol)))
             (core::intern symbol macro-fn env)))
            (core::compile-macro (core::list2 (mu:car lambda) (mu:cdr lambda)) env)))
       (mu:nth 1 form)
       (mu:nthcdr 2 form))))

(mu:intern core::ns :intern "defun"
   (:lambda (form env)
      ((:lambda (symbol lambda)
         (core:errorp-unless core:symbolp symbol "core::defun: not a symbol")
         (core:errorp-when core:keywordp symbol "core::defun: not a symbol")
         (core:errorp-unless core:listp lambda "core::defun: not a lambda list")
         (mu:compile (core::list2 :quote (core::intern symbol (core::lambda (mu:cons :lambda lambda) env) env))))
       (mu:nth 1 form)
       (mu:nthcdr 2 form))))

;;;
;;; this if form can take two or three arguments
;;;
(mu:intern core::ns :intern "if"
   (:lambda (form env)
      (core:errorp-unless core:listp env "core::if: not an environment")
      (core:errorp-unless (:lambda (form) (mu:fx-lt 2 (mu:length form))) form "core:if: takes two or three arguments")
      (core:errorp-unless (:lambda (form) (mu:fx-lt (mu:length form) 5)) form "core:if: takes two or three arguments")
      (core::compile
        ((:lambda (thunk if-impl)
            (mu:cons 'core:apply
              (mu:cons (thunk
                        (mu:cons if-impl
                          (mu:cons (core::compile (mu:nth 1 form) env)
                            (mu:cons (thunk (mu:nth 2 form))
                              (core::list (thunk (mu:nth 3 form)))))))
                '(()))))
         (:lambda (form)
            (core::compile
              (core::list3 'lambda () form)
              env))
         (:lambda (term true-fn false-fn)
            (:if term
                 (core:apply true-fn ())
                 (core:apply false-fn ()))))
        env)))

(mu:intern core::ns :intern "special-table"
   (:lambda (symbol)
      (mu:cdr (core:assoc
        symbol
        '((defconst    . core::defconst)
          (defun       . core::defun)
          (defmacro    . core::defmacro)
          (if          . core::if)
          (lambda      . core::lambda)
          (:quote      . core::quote))))))

;;;
;;; compile
;;;
(mu:intern core::ns :intern "compile-list"
   (:lambda (list env)
     (core:mapcar (:lambda (elt) (core::compile elt env)) list)))

(mu:intern core::ns :intern "compile"
    (:lambda (form env)
      (core:warn form "core::compile")
      (core:errorp-unless core:listp env "compile: env not a list")
      (:if (core:consp form)
          ((:lambda (fn args)
             (:if (mu:keyp fn)
                  (mu:compile form)     ;;; libmu special form (:key ...)
                  (:if (core:consp fn)  ;;; application ((...) ...), native or core
                       (core::compile-application fn args env)
                       ((:lambda (specf)
                          (:if specf
                               (mu:apply (mu:sy-val specf) (core::list2 form env))
                               (core::compile-application fn args env)))
                        (core::special-table fn)))))
           (mu:car form)
           (mu:cdr form))
          (:if (core:symbolp form)
               (core::compile-symbol form env)
               (mu:compile form)))))

(mu:intern core::ns :extern "compile"
   (:lambda (form)
     (core::compile form ())))
