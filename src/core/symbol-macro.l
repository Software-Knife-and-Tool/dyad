;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol macros
;;;
(mu:intern core::ns :extern "in-namespace"
   (:lambda (ns)
     (core:errorp-unless core:namespacep ns "core:in-namespace: not a namespace")
     (mu:make-ns ns)
     (core::read-set-ns ns)
     ns))
   
(mu:intern core::ns :intern "read-get-ns"
   (:lambda ()
     ((:lambda (name)
        (mu:write name core::reader-ns-stream ())
        (mu:map-ns name))
      (core:get-output-stream-string core::reader-ns-stream))))

(mu:intern core::ns :intern "read-set-ns"
   (:lambda (ns)
      (core:get-output-stream-string core::reader-ns-stream)
      (mu:write (mu:ns-name ns) () core::reader-ns-stream)))

;;;
;;; symbol readers
;;;
(mu:intern core::ns :intern "read-symbol-keywordp"
   (:lambda (name)
      (mu:eq #\: (core:schar name 0))))

(mu:intern core::ns :intern "read-symbol-externp"
   (:lambda (name)
      (mu:eq (core:positionl mu:eq #\: name) (core:positionr mu:eq #\: name))))

(mu:intern core::ns :intern "read-symbol-ns"
   (:lambda (name)
      ((:lambda (colon)
          (:if (core:null colon)
               ()
               (core:substr name 0 (core:1- colon))))
       (core:positionl mu:eq #\: name))))

(mu:intern core::ns :intern "read-symbol-name"
   (:lambda (name)
      ((:lambda (colon)
          (:if (core:null colon)
               name
               (core:substr
                 name
                 (core:1+ colon)
                 (core:1- (mu:sv-len name)))))
       (core:positionr mu:eq #\: name))))

(mu:intern core::ns :intern "read-resolve-symbol"
   (:lambda (symbol)
     (:if (core::read-symbol-keywordp symbol)
          (mu:keyword (core::read-symbol-name symbol))
          ((:lambda (ns scope base)
             (:if (core:null ns)
                  ((:lambda (sym)
                     (:if sym
                          sym
                          (mu:intern () :extern base)))
                   ((:lambda (walk) (walk walk (core::read-get-ns)))
                    (:lambda (walk ns)
                      (:if (core:namespacep ns)
                           ((:lambda (sym)
                              (:if (core:null sym)
                                   (walk walk (mu:ns-imp ns))
                                   sym))
                            (mu:ns-map ns :extern symbol))
                           ns))))
                  (mu:intern ns scope base)))
           ((:lambda (ns)
              (:if (core:null ns)
                   ()
                   (mu:map-ns ns)))
            (core::read-symbol-ns symbol))
           (:if (core::read-symbol-externp symbol)
                :extern
                :intern)
           (core::read-symbol-name symbol)))))

;;;
;;; symbol macros
;;;
(mu:intern core::ns :intern "core-symbol-macro-ns" (mu:make-ns "core:symbol-macro-ns" ()))
(mu:intern core::ns :intern "symbol-macro-expand"
   (:lambda (symbol)
     (:if (core:uninternedp symbol)
          ((:lambda (sym)
             (:if sym
                  (mu:eval (core:symbol-value sym))
                  symbol))
           (core::get-symbol-macro symbol))
          symbol)))

(mu:intern core::ns :intern "get-symbol-macro"
   (:lambda (symbol)
     (core:errorp-unless core:symbolp symbol "get-symbol-macro-form: not a symbol")
     ((:lambda (macro-symbol)
        (:if macro-symbol
            macro-symbol
            ()))
      (mu:ns-map core::core-symbol-macro-ns :extern (core:symbol-name symbol)))))

(mu:intern core::ns :extern "define-symbol-macro"
   (:lambda (symbol form)
     (core:errorp-unless core:symbolp symbol "define-symbol-macro: not a symbol")
     (mu:intern core::core-symbol-macro-ns :extern (mu:sy-name symbol) form)
     symbol))

;;; (core:define-symbol-macro 't :t)
;;; (core:define-symbol-macro 'nil :nil)
