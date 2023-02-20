;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core format
;;;
(mu:intern core::ns :intern "fmt-tilde"
  (:lambda (dest args)
    (core:write-char #\~ dest)
    args))

(mu:intern core::ns :intern "fmt-percent"
  (:lambda (dest args)
    (core:terpri dest)
    args))

(mu:intern core::ns :intern "fmt-decimal"
  (:lambda (dest args)
    (core:errorp-unless core:fixnump (mu:car args) "format: argument to ~D not a fixnum")
    (core:write (mu:car args) () dest)
    (mu:cdr args)))

(mu:intern core::ns :intern "fmt-hex"
  (:lambda (dest args)
    ((:lambda (fx)
       (core:errorp-unless core:fixnump fx "format: argument to ~X not a fixnum")
       (core:errorp-when (:lambda (n) (mu:fx-lt n 0)) fx "format: argument to ~X is negative")
       (:if (core:zerop fx)
            (core:write "0" () dest)
            ((:lambda (loop)
               ((:lambda (lst)
                  (core:mapc (:lambda (ch) (core:write ch () dest))
                             (mu:nthcdr (core:positionl
                                         (:lambda (item elt)
                                           (core:null (mu:eq item elt)))
                                         #\0
                                         lst)
                                        lst)))
                (loop loop fx 14)))
             (:lambda (loop fx nth)
               (:if (core:zerop nth)
                    ()
                    (core::append (loop loop (core:ash fx -4) (core:1- nth))
                                  (core::list (core:schar "0123456789abcdef" (mu:logand fx #xf))))))))
       (mu:cdr args))
     (mu:car args))))

(mu:intern core::ns :intern "fmt-aesthetic"
  (:lambda (dest args)
    (core:write (mu:car args) () dest)
    (mu:cdr args)))

(mu:intern core::ns :intern "fmt-standard"
  (:lambda (dest args)
    (core:write (mu:car args) :t dest)
    (mu:cdr args)))

;;;
;;; control ops
;;;
(mu:intern core::ns :intern "fmt-op"
  (:lambda (dir dest args)
    ((:lambda (fn)
       (:if fn
            (core:apply (mu:sy-val fn) (core::list2 dest args))
            (core:error dir "core:format: unrecognized format directive")))
     (mu:cdr
      (core:assoc
       dir
       '((#\A . core::fmt-aesthetic)
         (#\S . core::fmt-standard)
         (#\X . core::fmt-hex)
         (#\D . core::fmt-decimal)
         (#\~ . core::fmt-tilde)
         (#\% . core::fmt-percent)))))))

(mu:intern core::ns :intern "fmt-loop"
  (:lambda (stream dest fmt args)
    (mu:fix
     (:lambda (loop)
       ((:lambda (nth argl)
          (:if (mu:eq nth (core:length fmt))
               loop
               ((:lambda (ch)
                  (:if (mu:eq ch #\~)
                       (core::prog2
                           (core:error-if (mu:eq (core:1+ nth) (core:length fmt)) nth "core:format: unexpected end of string")
                           (mu:cons (mu:fx-add 2 nth) (core::fmt-op (core:schar fmt (core:1+ nth)) dest argl)))
                       (core::prog2
                           (core:write-char ch dest)
                           (mu:cons (core:1+ nth) argl))))
                (core:schar fmt nth))))
        (mu:car loop)
        (mu:cdr loop)))
     (mu:cons 0 args))
    (:if (core:null stream)
         (core:get-output-stream-string dest)
         ())))

(mu:intern core::ns :extern "format"
   (:lambda (stream fmt args)
     (core:errorp-unless core:stringp fmt "core:format: not a string")
     (core:errorp-unless core:listp args "core:format: not a list")

     (:if (core:zerop (core:length fmt))
          (:if stream
               ""
               ())
          ((:lambda (dest) (core::fmt-loop stream dest fmt args))
           (:if (core:null stream)
                (mu:open :string :output "")
                (:if (mu:eq stream :t)
                     mu:std-out
                     (:if (core:streamp stream)
                          stream
                          (core:error stream "core:format: not a stream designator"))))))))
