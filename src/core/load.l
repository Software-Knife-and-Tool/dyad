;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; loader
;;;
(mu:intern core::ns :extern "load"
  (:lambda (source verbose print)
    (core:raise-unless core:stringp source "core:load not a file designator")
    ((:lambda (ifs)
       (core:raise-unless core:streamp ifs "core:load cannot open input file")
       (:if verbose
            (core::prog2
                (core:write (core:string-append ";;; loading " source) () ()) 
                (core:terpri mu:std-out))
            ())
       (mu:fix
        (:lambda (loop)
          (:if (core:eof ifs)
               :t
               (mu:with-ex
                   (:lambda (obj cond)
                     (core:write "load exception" () mu:err-out)
                     (core:terpri mu:err-out)
                     (core:print-except cond "core:load")
                     (core:write ";;; in file " () mu:err-out)
                     (core:write source () mu:err-out)
                     (core:terpri mu:err-out))
                 (:lambda ()
                   ((:lambda (form)
                      (:if print
                           (core::prog2
                               (core:write form mu:std-out ())
                               (core:terpri mu:std-out))
                           ())
                      (core:null loop))
                    (mu:eval (core:compile (core:read ifs :t ()))))))))
        ()))
     (mu:open :file :input source))))

;;;
;;; debugging repl - no amenities
;;;
(mu:intern core::ns :intern "dev-repl"
  (:lambda ()
     (mu:fix            
      (:lambda (loop)
        (:if (core:eof :t)
             ()
             (mu:with-ex
                 (:lambda (obj cond)
                   (core::prog2
                       (core:print-exception (core:make-exception obj cond 'core::dev-repl))
                       (core:break ex))
                   (core:null loop))
               (:lambda ()
                 (core:format :t "core> " ())
                 (mu:flush mu:std-out)
                 ((:lambda (form)
                    (:if (core:eof :t)
                         ()
                         ((:lambda (form)
                            (core:format :t "[~A] ~A~%" (core::list2 (mu:type-of form) form))
                            (core:null loop))
                          (mu:eval (core:compile form)))))
                  (core:read :t () ()))))))
        ())))
