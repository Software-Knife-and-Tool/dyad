;;;  Spdx-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; exceptions
;;;
(mu::intern core::ns :extern "print-except"
   (:lambda (ex by)
     (core:write ";;; exception raised by " mu:err-out ())
     (core:write by mu:err-out ())
     (core:write " with " mu:err-out ())
     (core:write (mu::view ex) mu:err-out ())
     (core:terpri mu:err-out)
     (core:write ";;;     in " mu:err-out ())
     (core:write (mu::view ex) mu:err-out ())
     (core:terpri mu:err-out)))

(mu::intern core::ns :intern "print-frame"
   (:lambda (frame)
      ((:lambda (fn args)
         (core:format :t
                "(~A ~A)~%"
                (core::list2 (:if (core:symbolp (core::fn-form fn))
                              (core::fn-form fn)
                              (core::string-append :intern "lambda-" (core:symbol-name (core::fn-frame-id fn))))
                             (core:sv-list args))))
       (mu:car frame)
       (mu:cdr frame))))

;;;
;;; break loop
;;;
(mu::intern core::ns :extern "break"
   (:lambda (except)
     (core:errorp-unless core:exceptionp except "core:break: not an exception")
     (core:write ";;; entering break loop, :h for commands" () ())
     (core:terpri ())
     (mu::fix
      (:lambda (loop)
        (:if (core:exceptionp loop)
             loop
             (:if (core:eofp :t)
                  ex
                  ((:lambda (form)
                     (:if (mu:eq form :h)
                          ((:lambda ()
                             (core:write "break help:" () ()) (core:terpri ())
                             (core:write ":x - exit process" () ()) (core:terpri ())
                             (core:write ":r - return from break" () ()) (core:terpri ())
                             (core:write ":s - print stack frames" () ()) (core:terpri ())
                             (core:null loop)))
                          (:if (mu:eq form :x)
                               (system:exit 0)
                               (:if (mu:eq form :s)
                                    ((:lambda ()
                                       (core:mapc
                                        (:lambda (frame) (core::print-frame frame))
                                        (mu:car (env:context)))
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

;;;
;;; utilities
;;;
(mu::intern core::ns :extern "error"
   (:lambda (obj msg)
     (core:write "error: " mu:err-out ())
     (core:write msg mu:err-out ())
     (core:write " " mu:err-out ())
     (core:write obj mu:err-out :t)
     (core:terpri mu:err-out)
     (mu:raise (mu:except :co-err :error :cond obj))))

(mu::intern core::ns :extern "error-if"
   (:lambda (test obj msg)
     (:if test
          ((:lambda ()
             (core:write "error: " mu:err-out ())
             (core:write msg mu:err-out ())
             (core:write " " mu:err-out ())
             (core:write obj mu:err-out :t)
             (core:terpri mu:err-out)
             (mu:raise (mu:except :co-err :error :cond obj))))
          obj)))

(mu::intern core::ns :extern "errorp-when"
   (:lambda (predicate obj msg)
     (:if (predicate obj)
          ((:lambda ()
             (core:write "error: " mu:err-out ())
             (core:write msg mu:err-out ())
             (core:write " " mu:err-out ())
             (core:write obj mu:err-out :t)
             (core:terpri mu:err-out)
             (mu:raise (mu:except :co-err :error :cond obj))))
          obj)))

(mu::intern core::ns :extern "errorp-unless"
   (:lambda (predicate obj msg)
     (:if (predicate obj)
          obj
          ((:lambda ()
             (core:write "error: " mu:err-out ())
             (core:write msg mu:err-out ())
             (core:write " " mu:err-out ())
             (core:write obj mu:err-out :t)
             (core:terpri mu:err-out)
             (mu:raise (mu:except :co-err :error :cond obj)))))))

(mu::intern core::ns :extern "warn"
   (:lambda (obj msg)
     (core:write "warn: " mu:err-out ())
     (core:write msg mu:err-out ())
     (core:write " " mu:err-out ())
     (core:write obj mu:err-out :t)
     (core:terpri mu:err-out)
     obj))

(mu::intern core::ns :extern "debug"
   (:lambda (obj msg)
     (core:write "    " mu:err-out ())           
     (core:write msg mu:err-out ())
     (core:write " " mu:err-out ())
     (core:write obj mu:err-out :t)
     (core:terpri mu:err-out)
     obj))
