;;;  Spdx-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; exceptions
;;;

;;;
;;; utilities
;;;
(mu:intern core::ns :extern "make-exception"
  (:lambda (obj condition source)
    (mu:struct :except
      (mu:cons obj
        (mu:cons condition
          (mu:cons source
            (core::list (mu::frames))))))))

(mu:intern core::ns :extern "print-exception"
  (:lambda (stream exception)
    ((:lambda (obj source condition)
       (core:format stream 
                    ";;; exception raised on ~A, condition ~A by ~A~%"
                    (core::list3 obj source condition)))
     (mu:sv-ref (mu:st-vec exception) 0)
     (mu:sv-ref (mu:st-vec exception) 1)
     (mu:sv-ref (mu:st-vec exception) 2))))
     
(mu:intern core::ns :intern "print-frames"
  (:lambda (frames)
    (core:mapc
      (:lambda (frame)
        (core:format :t "(~A ~A)~%" frame))
        frames)))

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

(mu:intern core::ns :extern "raise"
   (:lambda (obj msg)
     (core::eprinc "error: ")
     (core::eprinc msg)
     (core::eprinc " ")
     (core::eprint obj)
     (core:terpri mu:err-out)
     (mu:raise :error obj)))

(mu:intern core::ns :extern "raise-if"
   (:lambda (test obj msg)
     (:if test
          ((:lambda ()
             (core::eprinc "raise-if: ")
             (core::eprinc msg)
             (core::eprinc " ")
             (core::eprint obj)
             (core:terpri mu:err-out)
             (mu:raise :error obj)))
          obj)))

(mu:intern core::ns :extern "raise-when"
   (:lambda (fn obj msg)
     (:if (mu:apply fn (core::list obj))
          ((:lambda ()
             (core::eprinc "raise-when: ")
             (core::eprinc msg)
             (core::eprinc " ")
             (core::eprint obj)
             (core:terpri mu:err-out)
             (mu:raise :error obj)))
          obj)))

(mu:intern core::ns :extern "raise-unless"
   (:lambda (fnp obj msg)
     (:if (mu:apply fnp (core::list obj))
          obj
          ((:lambda ()
             (core::eprinc "raise-unless: ")
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
     (core:raise-unless core:stringp except "core:break: not an exception")
     (core:write ";;; entering break loop, :h for commands" () ())
     (core:terpri ())
     (mu:fix
      (:lambda (loop)
        (:if (core:stringp loop)
             loop
             (:if (core:eof :t)
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
