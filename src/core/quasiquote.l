;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; quasiquote - list construction DSL
;;;
(mu:intern core::ns :intern "read-quasi-comma"
    (:lambda (stream)
      ((:lambda (at)
         (:if (mu:eq #\@ at) ;;; ,@
              (core::read stream)
              (core::prog2    ;;; ,
                  (core:unread-char at stream)
                 (core::list3 'mu:cons (core::read stream) ()))))
       (core:read-char stream () ()))))

(mu:intern core::ns :intern "read-quasi-quote"
   (:lambda (stream ch)
     ((:lambda (ch)
        (:if (mu:eq ch #\()
             ((:lambda (quasi-list)
                ;;; (core:warn quasi-list "quasi-list")
                (core:warn
                 (core::list2
                  'core:append
                  (core::flat-arg-list quasi-list))
                 "output-quasi-list"))
              ((:lambda ()
                 (mu:fix
                  (:lambda (self toggle end lst)
                    (:if end
                         lst
                         ((:lambda (comma)
                            (:if (mu:eq #\, comma)
                                 ((:lambda (form)
                                    (core:warn comma "processing a comma form")
                                    (:if (mu:eq form core::read-list-eol)
                                         (mu:fr-setv (core::fn-frame-id self) 2 :t)
                                         (mu:fr-setv (core::fn-frame-id self) 3
                                                      (core::append lst (core::list form)))))
                                    (core::read-quasi-comma stream))
                                 ((:lambda (form)
                                    (core:warn comma "processing a non-comma form")
                                    (:if (mu:eq form core::read-list-eol)
                                         (mu:fr-setv (core::fn-frame-id self) 2 :t)
                                         (mu:fr-setv (core::fn-frame-id self) 3
                                                      (core::append lst (core::list form)))))
                                    ((:lambda ()
                                       (core:unread-char comma stream)
                                       (core::read stream)))))
                            (mu:fr-setv (core::fn-frame-id self) 1 (core:null toggle)))
                          (core::read-consume-ws stream))))
                  '(() () ())))))
             (core::prog2
                (core:unread-char ch stream)
                (core::read stream))))
      (core:read-char stream () ()))))

(core:set-macro-character #\` core::read-quasi-quote :t)
