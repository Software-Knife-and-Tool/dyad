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
  (:lambda (obj cond)
     (mu:raise (core:make-exception obj cond "core:raise") :except)))

(mu:intern core::ns :extern "raise-if"
   (:lambda (test obj msg)
     (:if test
          (mu:raise (core:make-exception obj :error msg) :except)
          obj)))

(mu:intern core::ns :extern "raise-when"
   (:lambda (fn obj msg)
     (:if (mu:apply fn (core::list obj))
          (mu:raise (core:make-exception obj :error msg) :except)
          obj)))

(mu:intern core::ns :extern "raise-unless"
   (:lambda (fn obj msg)
     (:if (mu:apply fn (core::list obj))
          obj
          (mu:raise (core:make-exception obj :error msg) :except))))

(mu:intern core::ns :extern "warn"
  (:lambda (obj msg)
     (core:format mu:err-out "warning: ~A ~A~%" (core::list2 msg obj))
     obj))

;;;
;;; core repl - no amenities
;;;
(mu:intern core::ns :intern "repl"
  (:lambda ()
     (mu:fix            
      (:lambda (loop)
        (:if (core:eof :t)
             ()
             (mu:with-ex
                 (:lambda (obj cond)
                   (:if (mu:eq cond :except)
                        ((:lambda ()
                           (core::eprinc "core exception: ")
                           (core::print-exception mu:err-out obj)
                           (core:terpri mu:err-out)))
                        (core:format mu:err-out "mu exception: on ~S with condition ~A~%" (core::list2 obj cond)))
                   (core:null loop))
               (:lambda ()
                 (core:format :t "core> " ())
                 (mu:flush mu:std-out)
                 ((:lambda (form)
                    (:if (core:eof :t)
                         ()
                         ((:lambda (form)
                            (core:format :t "[~A] ~S~%" (core::list2 (mu:type-of form) form))
                            (core:null loop))
                          (mu:eval (core:compile form)))))
                  (core:read :t () ()))))))
        ())))
