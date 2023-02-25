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
(mu:intern core::ns :intern "read-char-syntax"
   (:lambda (ch)
     ((:lambda (read-table) (mu:cdr (core:assoc ch read-table)))
      '((#\return . :wspace) (#\linefeed . :wspace) (#\page . :wspace)
        (#\tab . :wspace) (#\space . :wspace)
        (#\0 . :const)  (#\1 . :const)  (#\2 . :const)  (#\3 . :const)
        (#\4 . :const)  (#\5 . :const)  (#\6 . :const)  (#\7 . :const)
        (#\8 . :const)  (#\9 . :const)  (#\: . :const)  (#\< . :const)
        (#\> . :const)  (#\= . :const)  (#\? . :const)  (#\! . :const)
        (#\@ . :const)  (#\; . :tmacro) (#\" . :tmacro) (#\# . :macro)
        (#\' . :tmacro) (#\( . :tmacro) (#\) . :tmacro) (#\` . :tmacro)
        (#\, . :tmacro) (#\\ . :escape) (#\| . :mescape) (#\a . :const)
        (#\b . :const)  (#\c . :const)  (#\d . :const)  (#\e . :const)
        (#\f . :const)  (#\g . :const)  (#\h . :const)  (#\i . :const)
        (#\j . :const)  (#\k . :const)  (#\l . :const)  (#\m . :const)
        (#\n . :const)  (#\o . :const)  (#\p . :const)  (#\q . :const)
        (#\r . :const)  (#\s . :const)  (#\t . :const)  (#\v . :const)
        (#\w . :const)  (#\x . :const)  (#\y . :const)  (#\z . :const)
        (#\[ . :const)  (#\] . :const)  (#\$ . :const)  (#\* . :const)
        (#\{ . :const)  (#\} . :const)  (#\+ . :const)  (#\- . :const)
        (#\/ . :const)  (#\~ . :const)  (#\. . :const)  (#\% . :const)
        (#\& . :const)  (#\^ . :const)  (#\_ . :const)  (#\a . :const)
        (#\b . :const)  (#\c . :const)  (#\d . :const)  (#\e . :const)
        (#\f . :const)  (#\g . :const)  (#\h . :const)  (#\i . :const)
        (#\j . :const)  (#\k . :const)  (#\l . :const)  (#\m . :const)
        (#\n . :const)  (#\o . :const)  (#\p . :const)  (#\q . :const)
        (#\r . :const)  (#\s . :const)  (#\t . :const)  (#\u . :const)
        (#\v . :const)  (#\w . :const)  (#\x . :const)  (#\y . :const)
        (#\z . :const)))))

;;;
;;; number reader
;;;
(mu:intern core::ns :intern "parse-integer"
  (:lambda (digits base)
    (:if (core:zerop (core:length digits))
         ()
         ((:lambda (sign)
            ((:lambda (integer)
               (:if (core:fixnump integer)
                    (:if (core:null sign)
                         integer
                         (mu:fx-mul sign integer))
                    ()))
             (mu:cdr
              (mu:fix
               (:lambda (loop)
                 (:if (core:null loop)
                      ()
                      ((:lambda (index acc)
                         (:if (mu:fx-lt (core:1- (core:length digits)) index)
                              loop
                              ((:lambda (n)
                                 (:if (core::logor (core:null n) (mu:fx-lt base n))
                                      ()
                                      (mu:cons (core:1+ index) (mu:fx-add n (mu:fx-mul acc base)))))
                               (core:string-positionl (:lambda (ch) (mu:eq ch (core:schar digits index))) "0123456789abcdef"))))
                       (mu:car loop)
                       (mu:cdr loop))))
               (:if (core:fixnump sign)
                    '(1 . 0)
                    '(0 . 0))))))
          ((:lambda (digit)
             (:if (mu:eq #\- digit)
                  -1
                  (:if (mu:eq #\+ digit)
                       1
                       ())))
           (core:schar digits 0))))))

(mu:intern core::ns :intern "parse-float"
  (:lambda (str)
    1.0))

(mu:intern core::ns :intern "read-number"
  (:lambda (atom)
    (core:warn atom "read-number")
     ((:lambda (fx)
        (:if fx
             fx
             (core::parse-float atom)))
      (core::parse-integer atom 10))))

;;;
;;; atom reader
;;;
(mu:intern core::ns :intern "read-atom"
   (:lambda (stream)
    (core:errorp-when mu:eof stream "core::read-atom: unexpected eof")
    (mu:car
     (mu:fix
      (:lambda (loop)
        (:if (core:consp loop)
             loop
             (:if (mu:eof stream)
                  ((:lambda (token)
                     ((:lambda (number)
                        (:if number
                             (core::list number)
                             (core::list (core::symbol-macro-expand (core::read-resolve-symbol token)))))
                      (core::read-number token)))
                   (core:get-output-stream-string core::reader-stream))
                  ((:lambda (ch)
                     (:if (mu:eq :const (core::read-char-syntax ch))
                          ((:lambda ()
                             (core:write-char ch core::reader-stream)
                             (core:null loop)))
                          (core::prog2
                              (core:unread-char ch stream)
                              ((:lambda (token)
                                 ((:lambda (number)
                                    (:if number
                                         (core::list number)
                                         (core::list (core::symbol-macro-expand (core::read-resolve-symbol token)))))
                                  (core::read-number token)))
                               (core:get-output-stream-string core::reader-stream)))))
                   (core:read-char stream () ())))))
      ()))))

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
      '((:const   . core::read-atom)
        (:escape  . core::read-atom)
        (:macro   . core::read-macro)
        (:tmacro  . core::read-macro)
        (:mescape . core::read-atom)))))

;;;
;;; core reader
;;;
(mu:intern core::ns :intern "read-consume-ws"
   (:lambda (stream)
     (mu:fix
      (:lambda (loop)
        (:if (core::logor (core:streamp loop) (core:charp loop))
             loop
             ((:lambda (ch)
                (:if (core:null ch)
                     stream
                     (:if (mu:eq :wspace (core::read-char-syntax ch))
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

;;;
;;; recursive reader
;;;
(mu:intern core::ns :intern "read"
  (:lambda (stream)
    (core:errorp-unless core:streamp stream "core:read: not a stream")
    ((:lambda (ch)
       (:if (mu:eof stream)
            ()
            ((:lambda (macro)
               (:if macro
                    (core:apply (mu:car macro) (core::list2 stream ch))
                    (core::read-dispatch ch stream)))
             (core:get-macro-character ch))))
     (core::read-consume-ws stream))))
