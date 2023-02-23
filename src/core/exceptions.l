;;;  Spdx-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; exceptions
;;;
(mu:intern core::ns :extern "print-except"
   (:lambda (ex by)
     (core:write ";;; exception raised by " ()  mu:err-out)
     (core:write by () mu:err-out)
     (core:write " with " () mu:err-out)
     (core:write (mu:view ex) () mu:err-out)
     (core:terpri mu:err-out)
     (core:write ";;;     in " () mu:err-out)
     (core:write (mu::view ex) () mu:err-out)
     (core:terpri mu:err-out)))

(mu:intern core::ns :intern "print-frame"
   (:lambda (frame)
      ((:lambda (fn args)
         (core:format :t
                "(~A ~A)~%"
                (core::list2
                 (:if (core:symbolp (core::fn-form fn))
                      (core::fn-form fn)
                      (core:string-append :intern "lambda-" (core:symbol-name (core::fn-frame-id fn))))
                 (core:sv-list args))))
       (mu:car frame)
       (mu:cdr frame))))

;;;
;;; utilities
;;;
(mu:intern core::ns :intern "eprinc"
  (:lambda (obj)
    (mu:write obj () mu:err-out)))           

(mu:intern core::ns :intern "eprint"
  (:lambda (obj)
    (mu:write obj :t mu:err-out)))           

(mu:intern core::ns :intern "princ"
  (:lambda (obj)
    (mu:write obj () mu:std-out)))           

(mu:intern core::ns :intern "print"
  (:lambda (obj)
    (mu:write obj :t mu:std-out)))           

(mu:intern core::ns :extern "error"
   (:lambda (obj msg)
     (core::eprinc "error: ")
     (core::eprinc msg)
     (core::eprinc " ")
     (core::eprint obj)
     (core:terpri mu:err-out)
     (mu:raise :error obj)))

(mu:intern core::ns :extern "error-if"
   (:lambda (test obj msg)
     (:if test
          ((:lambda ()
             (core::eprinc "error-if: ")
             (core::eprinc msg)
             (core::eprinc " ")
             (core::eprint obj)
             (core:terpri mu:err-out)
             (mu:raise :error obj)))
          obj)))

(mu:intern core::ns :extern "errorp-when"
   (:lambda (fn obj msg)
     (:if (mu:apply fn (core::list obj))
          ((:lambda ()
             (core::eprinc "errorp-when: ")
             (core::eprinc msg)
             (core::eprinc " ")
             (core::eprint obj)
             (core:terpri mu:err-out)
             (mu:raise :error obj)))
          obj)))

(mu:intern core::ns :extern "errorp-unless"
   (:lambda (fnp obj msg)
     (:if (mu:apply fnp (core::list obj))
          obj
          ((:lambda ()
             (core::eprinc "errorp-unless: ")
             (core::eprinc msg)
             (core::eprinc " ")
             (core::eprint obj)
             (core:terpri mu:err-out)
             (mu:raise :error obj))))))

(mu:intern core::ns :extern "warn"
   (:lambda (obj msg)
     (core::eprinc "warning: ")
     (core::eprinc msg)
     (core::eprinc " ")
     (core::eprint obj)
     (core:terpri mu:err-out)
     obj))

;;;
;;; break loop
;;;
(mu:intern core::ns :extern "break"
   (:lambda (except)
     (core:errorp-unless core:stringp except "core:break: not an exception")
     (core:write ";;; entering break loop, :h for commands" () ())
     (core:terpri ())
     (mu:fix
      (:lambda (loop)
        (:if (core:stringp loop)
             loop
             (:if (core:eofp :t)
                  ()
                  ((:lambda (form)
                     (:if (mu:eq form :h)
                          ((:lambda ()
                             (core:write "break help:" () ()) (core:terpri ())
                             (core:write ":x - exit process" () ()) (core:terpri ())
                             (core:write ":r - return from break" () ()) (core:terpri ())
                             (core:write ":s - print stack frames" () ()) (core:terpri ())
                             (core:null loop)))
                          (:if (mu:eq form :x)
                               (mu:exit 0)
                               (:if (mu:eq form :s)
                                    ((:lambda ()
                                       (core:mapc
                                        (:lambda (frame) (core::print-frame frame))
                                        (mu:car (mu::context)))
                                       (core:null loop)))
                                    (:if (mu:eq form :r)
                                         except
                                         ((:lambda ()
                                            (core:write (mu:eval (core:compile form)) () :t)
                                            (core:terpri ())
                                            (core:null loop))))))))
                   ((:lambda ()
                      (core:write "break> " () ())
                      (core:read :t () ())))))))
     ())))
