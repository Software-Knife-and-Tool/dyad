;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; loader
;;;
(mu:intern core::ns :extern "load"
  (:lambda (source verbose print)
    (core:errorp-unless core:stringp source "core:load not a file designator")
    ((:lambda (ifs)
       (core:errorp-unless core:streamp ifs "core:load cannot open input file")
       (:if verbose
            (core::prog2
                (core:write (core::string-append ";;; loading " source) mu:std-out ()) 
                (core:terpri mu:std-out))
            ())
       (mu:fix
        (:lambda (loop)
          (:if (mu:eof ifs)
               :t
               (mu:with-ex
                   (:lambda (cond obj)
                     (core:write "load exception" mu:err-out ())
                     (core:terpri mu:err-out)
                     (core:print-except cond "core:load")
                     (core:write ";;; in file " mu:err-out ())
                     (core:write source mu:err-out ())
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
(mu:intern core::ns :intern "debug-repl"
   (:lambda ()
     ((:lambda (repl) (repl repl))
      (:lambda (repl)
        (mu:with-ex
           (:lambda (ex)
             (:if (core:eofp :t)
                  (mu::exit 0)
                  (core::prog2
                     (core:print-except ex "core::debug-repl")
                     (core:break ex))))
          (:lambda ()
            (core:format :t "core> " ())
            ((:lambda (form)
               (:if (core:eofp :t)
                    (core::prog2
                       (core:format :t "sic transit gloria mundi~%" ())
                       (mu::exit 0))
                    ((:lambda (form)
                       (core:format :t "[~A] ~A~%" (core::list2 (mu:type-of form) form)))
                     (mu:eval (core:compile form)))))
             (core:read :t :t :t))))
        (repl repl)))))
