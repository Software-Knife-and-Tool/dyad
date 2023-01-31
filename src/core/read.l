;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core reader
;;;
(mu:intern core::ns :intern "reader-stream" (mu:open :string :output ""))
(mu:intern core::ns :intern "reader-ns-stream" (mu:open :string :output "core"))

;;;
;;; readtable
;;;
(mu:intern core::ns :intern "read-whitespace"
    (mu:cons (mu:cons (mu:coerce #xa :char) 'wspace)
      (mu:cons (mu:cons (mu:coerce #xd :char) 'wspace)
        (mu:cons (mu:cons (mu:coerce #xc :char) 'wspace)
          (core::list (mu:cons (mu:coerce #x20 :char) 'wspace))))))

(mu:intern core::ns :intern "read-char-syntax"
   (:lambda (ch)
     ((:lambda (read-table)
         (mu:cdr (core:assoc ch read-table)))
      (core::append
       core::read-whitespace
       '((#\0 . constituent)  (#\1 . constituent)  (#\2 . constituent)  (#\3 . constituent)
         (#\4 . constituent)  (#\5 . constituent)  (#\6 . constituent)  (#\7 . constituent)
         (#\8 . constituent)  (#\9 . constituent)  (#\: . constituent)  (#\< . constituent)
         (#\> . constituent)  (#\= . constituent)  (#\? . constituent)  (#\! . constituent)
         (#\@ . constituent)  (#\; . tmacro)       (#\" . tmacro)       (#\# . macro)
         (#\' . tmacro)       (#\( . tmacro)       (#\) . tmacro)       (#\` . tmacro)
         (#\, . tmacro)       (#\\ . escape)       (#\| . mescape)      (#\a . constituent)
         (#\b . constituent)  (#\c . constituent)  (#\d . constituent)  (#\e . constituent)
         (#\f . constituent)  (#\g . constituent)  (#\h . constituent)  (#\i . constituent)
         (#\j . constituent)  (#\k . constituent)  (#\l . constituent)  (#\m . constituent)
         (#\n . constituent)  (#\o . constituent)  (#\p . constituent)  (#\q . constituent)
         (#\r . constituent)  (#\s . constituent)  (#\t . constituent)  (#\v . constituent)
         (#\w . constituent)  (#\x . constituent)  (#\y . constituent)  (#\z . constituent)
         (#\[ . constituent)  (#\] . constituent)  (#\$ . constituent)  (#\* . constituent)
         (#\{ . constituent)  (#\} . constituent)  (#\+ . constituent)  (#\- . constituent)
         (#\/ . constituent)  (#\~ . constituent)  (#\. . constituent)  (#\% . constituent)
         (#\& . constituent)  (#\^ . constituent)  (#\_ . constituent)  (#\a . constituent)
         (#\b . constituent)  (#\c . constituent)  (#\d . constituent)  (#\e . constituent)
         (#\f . constituent)  (#\g . constituent)  (#\h . constituent)  (#\i . constituent)
         (#\j . constituent)  (#\k . constituent)  (#\l . constituent)  (#\m . constituent)
         (#\n . constituent)  (#\o . constituent)  (#\p . constituent)  (#\q . constituent)
         (#\r . constituent)  (#\s . constituent)  (#\t . constituent)  (#\u . constituent)
         (#\v . constituent)  (#\w . constituent)  (#\x . constituent)  (#\y . constituent)
         (#\z . constituent))))))

;;;
;;; number reader
;;;
(mu:intern core::ns :intern "read-number"
   (:lambda (atom)
     ((:lambda (fx)
        (:if fx
             fx
             (mu::fparse atom)))
      (mu::iparse atom 10))))

;;;
;;; atom reader
;;;
(mu:intern core::ns :intern "read-atom"
   (:lambda (stream)
     (core:errorp-when mu:eof stream "core::read-atom: unexpected eof")
     (mu:fix*           
      (:lambda (self end atom)
        (:if (core::logical-or (core:fixnump end) (mu:eof stream))
             atom
             ((:lambda (ch)
                (:if (mu:eq 'constituent (core::read-char-syntax ch))
                     (core::prog2
                        (core:write-char ch core::reader-stream)
                        (mu:fr-setv (core::fn-frame-id self) 1 (core:null end)))
                     ((:lambda ()
                        (core:unread-char ch stream)
                        (mu:fr-setv (core::fn-frame-id self) 1 0)
                        (mu:fr-setv (core::fn-frame-id self) 2
                                     ((:lambda (value)
                                        ((:lambda (number)
                                           (:if number
                                                number
                                                (core::symbol-macro-expand (core::read-resolve-symbol value))))
                                         (core::read-number value)))
                                      (core:get-output-stream-string core::reader-stream)))
                        atom))))
              (core:read-char stream () ()))))
      '(() ()))))

;;;
;;; parser
;;;
(mu:intern core::ns :intern "read-dispatch"
   (:lambda (ch stream)
     (core:unread-char ch stream)
     ((:lambda (dispatch-table)
        (mu:apply
         (mu:sy-val (mu:cdr (core:assoc (core::read-char-syntax ch) dispatch-table)))
         (core::list stream)))
      '((constituent . core::read-atom)
        (macro       . core::read-macro)
        (tmacro      . core::read-macro)
        (escape      . core::read-atom)
        (mescape     . core::read-atom)))))

;;;
;;; core reader
;;;
(mu:intern core::ns :intern "read-consume-ws"
   (:lambda (stream)
     (mu:fix
      (:lambda (loop)
        (:if (core::logical-or (core:streamp loop) (core:charp loop))
             loop
             ((:lambda (ch)
                (:if (mu:eof stream)
                     stream
                     (:if (mu:eq 'wspace (core::read-char-syntax ch))
                          (core:null loop)
                          (:if (mu:eq ch #\#)
                               ((:lambda (ch)
                                  (:if (mu:eq ch #\|)
                                       (core::prog2
                                          (core::read-sharp-comment ch stream)
                                          (core:null loop))
                                       (core::prog2
                                          (core:unread-char ch stream)
                                          #\#)))
                                (core:read-char stream () ()))
                               (:if (mu:eq ch #\;)
                                    (core::prog2
                                       (core::read-line-comment ch stream)
                                       (core:null loop))
                                    ch)))))
              (core:read-char stream () ()))))
      ())))



(mu:intern core::ns :intern "read"
   (:lambda (stream)
     (core:errorp-unless core:streamp stream ":read: not a stream")
     ((:lambda (read-consume-ws)
        ((:lambda (ch)
           (:if (mu:eof stream)
                ()
                ((:lambda (macro)
                   (:if macro
                        (core::apply (mu:car macro) (core::list2 stream ch))
                        (core::read-dispatch ch stream)))
                 (core:get-macro-character ch))))
         (read-consume-ws)))
      (:lambda ()
        (mu:fix
         (:lambda (loop)
           (:if (core::logical-or (core:streamp loop) (core:charp loop))
                loop
                ((:lambda (ch)
                   (:if (mu:eof stream)
                        stream
                        (:if (mu:eq 'wspace (core::read-char-syntax ch))
                             (core:null loop)
                             (:if (mu:eq ch #\#)
                                  ((:lambda (ch)
                                     (:if (mu:eq ch #\|)
                                          (core::prog2
                                             (core::read-sharp-comment ch stream)
                                             (core:null loop))
                                          (core::prog2
                                             (core:unread-char ch stream)
                                             #\#)))
                                   (core:read-char stream () ()))
                                  (:if (mu:eq ch #\;)
                                       (core::prog2
                                          (core::read-line-comment ch stream)
                                          (core:null loop))
                                       ch)))))
                 (core:read-char stream () ()))))
         ())))))
