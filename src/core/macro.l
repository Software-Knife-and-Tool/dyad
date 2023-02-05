;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; macro definition, compilation, and expansion
;;;

;;;
;;; Macro definitions are compiled like lambdas (though eventually with a slighlty
;;; different lambda list including &whole).
;;;
;;; Macro calls are expanded at compile time.
;;;
;;; Common Lisp adds an environment argument here for lexial macro expanders.
;;;
;;; an environment is a closure that evaluates its argument.
;;;

;;;
;;; macro environment
;;;
(mu:intern core::ns :intern "core-macro-env"
   (:lambda (symbol macro-fn)
     ((:lambda (macro-ns symbol-ns symbol-name symbol-scope)
        (:if macro-ns
             (mu:intern (mu:sy-val macro-ns) symbol-scope symbol-name macro-fn)
             ((:lambda (ns-sym)
                (mu:intern (mu:sy-val ns-sym) symbol-scope symbol-name macro-fn))
              (mu:intern
               symbol-ns
               :intern
               (core::string-append (mu:ns-name symbol-ns) ":macrodefs")
               (mu:make-ns (mu:make-ns (core::string-append (mu:ns-name symbol-ns) ":macro-functions") ()))))))
      (mu:ns-map (mu:sy-ns symbol) :intern (core::string-append (mu:ns-name (mu:sy-ns symbol)) ":macrodefs"))
      (mu:sy-ns symbol)
      (mu:sy-name symbol)
      (mu:sy-ext symbol))))

(mu:intern core::ns :intern "core-find-macro-env"
   (:lambda (symbol env)
     (core:errorp-unless core:symbolp symbol "core-find-macro-env: is not a symbol")           
     ((:lambda (macro-ns symbol-ns symbol-name symbol-scope)
        (:if macro-ns
             (mu:ns-map (mu:sy-val macro-ns) symbol-scope symbol-name)
             ()))
     (mu:ns-map (mu:sy-ns symbol) :intern (core::string-append (mu:ns-name (mu:sy-ns symbol)) ":macrodefs"))
     (mu:sy-ns symbol)
     (mu:sy-name symbol)
     (mu:sy-ext symbol))))

;;;
;;; core:macroexpand-1 expands a macro call once.
;;;
(mu:intern core::ns :intern "macroexpand-1"
   (:lambda (form env)
     (:if (core:consp form)
          ((:lambda (fn-symbol args)
             (:if (core:symbolp fn-symbol)
                  ((:lambda (macro-fn)
                     (:if macro-fn
                          (core::apply macro-fn args)
                          form))
                   (core:macro-function fn-symbol env))
                  form))
           (mu:car form)
           (mu:cdr form))
          form)))

;;;
;;; core:macroexpand: expand a form until it is no longer a macro call
;;;
(mu:intern core::ns :extern "macroexpand"
   (:lambda (form env)
     ((:lambda (expanded)
          (:if (mu:eq form expanded)
             expanded
             (core:macroexpand expanded env)))
       (core::macroexpand-1 form env))))

;;;
;;; core:macro-function: map a symbol to a macro function
;;;
;;; acts as a predicate
;;;
(mu:intern core::ns :extern "macro-function"
  (:lambda (symbol env)
     (:if (core:symbolp symbol)            
          (:if (core:keywordp symbol)
               ()
               ((:lambda (macro)
                  (:if macro
                       (mu:sy-val macro)
                       ()))
                (core::core-find-macro-env symbol env)))
          ())))

;;;
;;; core::compile-macro form env => function
;;;
;;; core::compile-macro compiles a macro expander function.
;;;
(mu:intern core::ns :intern "core-macro"
   (:lambda (form env)
      (:if (core:findl-if (:lambda (el) (core:null (core:symbolp el))) form)
         (core:error-if form "core::core-macro: lambda syntax")
         ((:lambda (desc)
             (:if (core:fixnump desc)
                  (core::lambda-descriptor form form () env ())
                  desc))
          (core:foldl
            (:lambda (el acc)
               (:if (core:vectorp acc)
                  acc
                  (:if (mu:eq '&rest el)
                     (:if (mu:eq (mu:length form) (mu:fx-addXs 2 acc))
                          (core::lambda-descriptor
                           (core::append (core:dropr form 2) (core:dropl form (core:1- (mu:length form))))
                           (core:dropr form 2)
                           (mu:nth (core:1- (mu:length form)) form)
                           env
                           ())
                          (core:error form "core::core-macro: rest botch"))
                     (core:1+ acc))))
            0
            form)))))

(mu:intern core::ns :intern "compile-macro"
   (:lambda (form env)
     ((:lambda (lambda-desc body)
        (mu:compile
         (mu:cons :lambda
                  (mu:cons (core::lambda-syms lambda-desc)
                           (core::compile-lambda-body lambda-desc body env)))))
      (core::core-macro (mu:car form) env)
      (mu:nth 1 form))))
