;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; strings
;;;
(mu:intern core::ns :extern "stringp"
   (:lambda (v)
      (:if (core:vectorp v)
           (mu:eq :char (mu:sv-type v))
           ())))

(mu:intern core::ns :extern "get-output-stream-string"
   (:lambda (stream)
      (mu:get-str stream)))

(mu:intern core::ns :extern "schar"
  (:lambda (str index)
    (core:errorp-unless core:stringp str "core:schar: not a string")
    (core:errorp-unless core:fixnump index "core:schar not a string")
    (mu:sv-ref str index)))

(mu:intern core::ns :extern "string="
   (:lambda (str-1 str-2)
      (core:errorp-unless core:stringp str-1 "core:string= not a string")
      (core:errorp-unless core:stringp str-2 "core:string= not a string")
      (:if (mu:eq str-1 str-2)
         :t
         (:if (mu:eq (mu:sv-len str-1) (mu:sv-len str-2))
              (mu:fix
               (:lambda (nth)
                 (:if (core:numberp nth)
                      (:if (mu:eq nth (mu:sv-len str-1))
                           :t
                           (:if (mu:eq (mu:sv-ref str-1 nth) (mu:sv-ref str-2 nth))
                                (core:1+ nth)
                                ()))
                      nth))
               0)
              ()))))

;;;
;;; string construction
;;;
(mu:intern core::ns :extern "string"
   (:lambda (designator)
      (:if (core:stringp designator)
         designator
         (:if (core:symbolp designator)
              (mu:sy-name designator)
              (:if (core:charp designator)
                   (mu:vector :char (core::list designator))
                   (core:error designator "core::string not a char"))))))

(mu:intern core::ns :extern "string-append"
   (:lambda (list)
     (core:errorp-unless core:listp list "core:string: is not a list")
     ((:lambda (result)
       (core:mapc
        (:lambda (str)
          (core:errorp-unless core:stringp str "core:string-append: is not a string")
          (mu:fix
           (:lambda (nth)
             (:if (mu:eq (mu:sv-len str) nth)
                  nth
                  (core::prog2
                      (mu:write (mu:sv-ref str nth) () result)
                      (core:1+ nth))))
           0))
        list)
       (mu:get-str result))
     (mu:open :string :output ""))))

(mu:intern core::ns :intern "substr"
  (:lambda (str start end)
     (core:errorp-unless core:stringp str "core:substr: is not a string")
     (core:errorp-unless core:fixnump start "core:substr: start is not a fixnum")
     (core:errorp-unless core:fixnump end "core:substr: end is not a fixnum")
     ((:lambda (substr)
        (mu:fix
         (:lambda (nth)
           (:if (core:numberp nth)
                (:if (mu:fx-lt nth end)
                     (mu:write (mu:sv-ref str nth) () substr)
                     ())
                nth))
         start)
        (mu:get-str substr))
        (mu:open :string :output ""))))
