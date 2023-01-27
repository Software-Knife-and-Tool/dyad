;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; strings
;;;
(mu::intern core::ns :extern "stringp"
   (:lambda (v)
      (:if (core:vectorp v)
           (mu:eq :char (mu:sv-type v))
           ())))

(mu::intern core::ns :extern "get-output-stream-string"
   (:lambda (stream)
      (mu:get-buf :char stream)))

(mu::intern core::ns :extern "schar"
  (:lambda (str index)
    (core:errorp-unless core:stringp str "core:schar: not a string")
    (core:errorp-unless core:fixnump index "core:schar not a string")
    (mu:sv-ref str index)))

(mu::intern core::ns :extern "string="
   (:lambda (str-1 str-2)
      (core:errorp-unless core:stringp str-1 "core:string= not a string")
      (core:errorp-unless core:stringp str-2 "core:string= not a string")
      (:if (mu:eq str-1 str-2)
         :t
         (:if (mu:eq (mu:sv-len str-1) (mu:sv-len str-2))
            ((:lambda (result)
               (core:charp result))
             (core:foldl
               (:lambda (el acc)
                  (:if (core:null acc)
                       ()
                       (:if (mu:eq el (mu:sv-ref str-2 acc))
                            (core:1+ acc)
                            ())))
                  0
                  str-1))
            ()))))

;;;
;;; string construction
;;;
(mu::intern core::ns :extern "string"
   (:lambda (designator)
      (:if (core:stringp designator)
         designator
         (:if (core:symbolp designator)
              (mu:sy-name designator)
              (:if (core:charp designator)
                   (mu:list-sv :char (core::list designator))
                   (core:error designator "core::string not a char"))))))

(mu::intern core::ns :extern "string-append"
   (:lambda (list)
      (core:errorp-unless core:listp list "core:string: is not a list")
      (mu:list-sv
       :char
       (core:foldl
        (:lambda (elt acc)
          (core:errorp-unless core:stringp elt "core:string: is not a string")
          (core::append acc (core:sv-list elt)))
        ()
        list))))

(mu::intern core::ns :intern "string-append"
   (:lambda (str-1 str-2)
      (core:errorp-unless core:stringp str-1 "is not a string (core::string)")
      (core:errorp-unless core:stringp str-2 "is not a string (core::string)")
      (mu:list-sv
        :char
        (core::append (core:sv-list str-1) (core:sv-list str-2)))))

;;; can use slice here?
(mu::intern core::ns :extern "substr"
  (:lambda (str start end)
     (core:errorp-unless core:stringp str "is not a string (core::substr)")
     (core:errorp-unless core:fixnump start "is not a fixnum (core::substr)")
     (core:errorp-unless core:fixnump end "is not a fixnum (core::substr)")
     ((:lambda (list)
         (mu:list-sv :char
           (core:dropl (core:dropr list (mu:fixnum- (mu:length list) (core:1+ end))) start)))
         (core:sv-list str))))
