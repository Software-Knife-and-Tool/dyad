;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; read macros
;;;

;;;
;;; sharpsign readers
;;;
(mu::intern core::ns :intern "read-sharp-char"
   (:lambda (ch stream)
      (core:read-char stream () ())))

(mu::intern core::ns :intern "read-sharp-comment"
    (:lambda (ch stream)
      (mu::fix
       (:lambda (loop)
         (:if (core:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (mu:eq ch #\|)
                      ((:lambda (ch)
                         (:if (mu:eq ch #\#)
                              stream
                              (core:null loop)))
                       (:if (mu:eofp stream)
                            (core:error stream "core::read-sharp-comment: unexpected end of file")
                            (core:read-char stream () ())))
                      (core:null loop)))
               (:if (mu:eofp stream)
                    (core:error stream "core::read-sharp-comment: unexpected end of file")
                    (core:read-char stream () ())))))
       ())))

(mu::intern core::ns :intern "read-sharp-vector"
    (:lambda (ch stream)
      ((:lambda (type)
          (core:errorp-unless mu:keysymp type "core:read-sharp-vector: not a type keyword")
          (mu:list-sv type (core::read-list #\( stream)))
       (core::read-atom stream))))

(mu::intern core::ns :intern "read-sharp-symbol"
   (:lambda (ch stream)
     ((:lambda (symbol)
          (core:errorp-unless core:symbolp symbol "core::read-sharp-symbol: not a symbol")
          (mu:symbol (mu:sy-name symbol)))
       (core::read-atom stream))))

(mu::intern core::ns :intern "read-sharp-number"
   (:lambda (base stream)
     (mu::fix           
      (:lambda (loop)
        (:if (core::logical-or (mu:eofp stream) (core:numberp loop))
             loop
             ((:lambda (ch)
                ((:lambda (syntax-type)
                   (:if (mu:eq syntax-type 'constituent)
                        (core::prog
                           (core:write-char ch core::reader-stream)
                           (core:null loop))
                        (core::prog
                           (:if (core:null ch)
                                ()
                                (core:unread-char ch stream))
                           (:if (mu:eq base #\x)
                                (mu::iparse (core:get-output-stream-string core::reader-stream) 16)
                                (:if (mu:eq base #\b)
                                     (mu::iparse (core:get-output-stream-string core::reader-stream) 2)
                                     (mu::iparse (core:get-output-stream-string core::reader-stream) 10))))))
                 (core::read-char-syntax ch)))
              (core:read-char stream () ()))))
      ())))
   
(mu::intern core::ns :intern "read-sharp"
   (:lambda (ch stream)
      ((:lambda (ch sharp-table)
          (mu:funcall
            (mu:sy-val (mu:cdr (core:assoc ch sharp-table)))
            (core::list2 ch stream)))
       (core:read-char stream () ())
       '((#\| . core::read-sharp-comment)
         (#\\ . core::read-sharp-char)
         (#\( . core::read-sharp-vector)
         (#\b . core::read-sharp-number)
         (#\x . core::read-sharp-number)
         (#\d . core::read-sharp-number)
         (#\: . core::read-sharp-symbol)))))

;;;
;;; list reader
;;;
(mu::intern core::ns :intern "read-list-eol" (mu:symbol "eol"))
#|
(mu::intern core::ns :intern "read-list"
    (:lambda (ch stream)
        (:if (mu:eq ch #\))
             core::read-list-eol
             (mu::fix*
              (:lambda (self eol list)
                (:if eol
                     list
                     ((:lambda (form)
                        (:if (mu:eq form core::read-list-eol)
                             (env:fp-setv (core::fn-frame-id self) 1 core::read-list-eol)
                             (:if (core:symbolp form)
                                  (:if (core::logical-and (core:uninternedp form) (mu:eq (mu:sy-name form) "."))
                                       ((:lambda (dotted)
                                          (core:error-if (core:null list) dotted "read-list: malformed dotted list")
                                          ((:lambda (eol)
                                             (:if (mu:eq eol core::read-list-eol)
                                                  (core::prog
                                                     (env:fp-setv (core::fn-frame-id self) 1 core::read-list-eol)
                                                     (env:fp-setv (core::fn-frame-id self) 2 (core::append list dotted)))
                                                  (core:error eol "core::read-list: malformed dotted list")))
                                           (core::read stream)))
                                        (core::read stream))
                                       (env:fp-setv (core::fn-frame-id self) 2 (core::append list (core::list form))))
                                  (env:fp-setv (core::fn-frame-id self) 2 (core::append list (core::list form))))))
                      (core::read stream))))
              '(() ())))))
|#

(mu::intern core::ns :intern "read-list"
   (:lambda (ch stream)
      ((:lambda (loop)
          (:if (mu:eq ch #\))
               core::read-list-eol
               (loop loop ())))
       (:lambda (loop list)
          ((:lambda (el)
              (:if (mu:eq el core::read-list-eol)
                   list
                   (:if (core:symbolp el)
                        (:if (mu:eq (mu:sy-name el) ".")
                             ((:lambda (dotted)
                                (core:error-if (core:null list) () "read-list: malformed dotted list")
                                (:if (mu:eq (core::read stream) core::read-list-eol)
                                  dotted
                                  (core:error () "read-list: malformed dotted list")))
                              (core::append list (core::read stream)))
                             (loop loop (core::append list (core::list  el))))
                        (loop loop (core::append list (core::list el))))))
           (core::read stream))))))

;;;
;;; string reader
;;;
(mu::intern core::ns :intern "read-string"
   (:lambda (ch stream)
     (mu::fix
      (:lambda (loop)
        (:if (core:stringp loop)
             loop
             ((:lambda (ch)
                (:if (mu:eq ch #\")
                     (core:get-output-stream-string core::reader-stream)
                     (core::prog
                        (core:write-char ch core::reader-stream)
                        (core:null loop))))
              (core:read-char stream () ()))))
        ())))

;;;
;;; quote reader
;;;
(mu::intern core::ns :intern "read-quote"
   (:lambda (ch stream)
     (core::list2 :quote (core::read stream))))

;;;
;;; line comment
;;;
(mu::intern core::ns :intern "read-line-comment"
    (:lambda (ch stream)
      (mu::fix
       (:lambda (loop)
         (:if (core:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (mu:eq ch (system:ansi-ch #xa))
                      stream
                      (core:null loop)))
               (core:read-char stream () ()))))
       ())))

;;;
;;; read macros
;;;
(mu::intern core::ns :intern "read-macro"
   (:lambda (stream)
      ((:lambda (ch macro-table)
          (mu:funcall
            (mu:sy-val (mu:cdr (core:assoc ch macro-table)))
            (core::list2 ch stream)))
       (core:read-char stream () ())
       '((#\" . core::read-string)
         (#\# . core::read-sharp)
         (#\' . core::read-quote)
         (#\( . core::read-list)
         (#\) . core::read-list)
         (#\; . core::read-line-comment)
         (#\, . core::quasiquote)
         (#\` . core::quasiquote)))))

;;;
;;; get/set-macro-character
;;;
(mu::intern core::ns :intern "core-char-macro-ns" (env:ns (mu:ns "core:char-macro-ns" ())))
(mu::intern core::ns :extern "get-macro-character"
   (:lambda (char)
     (core:errorp-unless core:charp char "get-macro-character: not a character")
     ((:lambda (macro-symbol)
        (:if macro-symbol
            (core:symbol-value macro-symbol)
            ()))
      (mu:ns-find core::core-char-macro-ns :extern (mu:list-sv :char (core::list char))))))

(mu::intern core::ns :extern "set-macro-character"
   (:lambda (char fn non-terminating)
     (core:errorp-unless core:charp char "set-macro-character: not a character")
     (core:errorp-unless core:functionp fn "set-macro-character: not a function")
     (mu::intern core::core-char-macro-ns :extern (mu:list-sv :char (core::list char)) (mu:cons fn non-terminating))
     :t))
