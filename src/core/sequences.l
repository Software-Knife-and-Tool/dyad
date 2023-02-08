;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; sequences
;;;
(mu:intern core::ns :extern "length"
   (:lambda (seq)
      (core:errorp-unless core:sequencep seq "core:length: not a sequence")
      (:if (core:listp seq)
           (mu:length seq)
           (mu:sv-len seq))))

(mu:intern core::ns :extern "elt"
   (:lambda (seq nth)
      (core:errorp-unless core:sequencep seq "core:elt: not a sequence")
      (core:errorp-unless core:fixnump nth "core:elt: not a fixnum")
      (:if (core:listp seq)
           (mu:nth nth seq)
           (mu:sv-ref seq nth))))

;;;
;;; finds
;;;
(mu:intern core::ns :extern "findl-if"
   (:lambda (fn seq)
     (core:errorp-unless core:functionp fn "core:findl-if: not a function")
     (core:errorp-unless core:sequencep seq "core:findl-if: not a sequence")
     (:if (core::fn-applyp fn)
      (core:foldl
       (:lambda (el acc)
         (:if (core::apply fn (core::list el)) el acc))
       ()
       seq)
      (core:foldl
       (:lambda (el acc) (:if (fn el) el acc))
       ()
       seq))))

(mu:intern core::ns :extern "findr-if"
   (:lambda (fn list)
     (core:errorp-unless core:functionp fn "core:findr-if not a function")
     (core:errorp-unless core:sequencep list "core:findr-if not a sequence")
     (:if (core::fn-applyp fn)
          (core:foldr
           (:lambda (el acc)
             (:if (core::apply fn (core::list el)) el acc))
           ()
           list)
          (core:foldr
           (:lambda (el acc) (:if (fn el) el acc))
           ()
           list))))

;;;
;;; position
;;;
(mu:intern core::ns :extern "positionl"
   (:lambda (fn item seq)
     (core:errorp-unless core:functionp fn "core:positionl: not a function")
     (core:errorp-unless core:sequencep seq "core:positionl: not a sequence")
     (:if (core::fn-applyp fn)
          (:if (core:listp seq)
               (:if (core:null seq)
                    ()
                    ((:lambda (lst)
                       (:if (core:null lst)
                            ()
                            (mu:fx-sub (mu:length seq) (mu:length lst))))
                     (mu:fix
                      (:lambda (list)
                        (:if (core:null list)
                             ()
                             (:if (core::apply fn (mu:cons item (mu:cons (mu:car list) 0)))
                                  list
                                  (mu:cdr list))))
                      seq)))
               (:if (core:zerop (core:length seq))
                    ()
                    ((:lambda (nth)
                       (:if (core:null nth)
                            ()
                            nth))
                     (mu:fix
                      (:lambda (nth)
                        (:if (core:null nth)
                             ()
                             (:if (mu:fx-lt nth (mu:sv-len seq))
                                  (:if (core::apply fn (core::list2 item (mu:sv-ref seq nth)))
                                       nth
                                       (core:1+ nth))
                                  ())))
                      0))))
          (:if (core:listp seq)
               (:if (core:null seq)
                    ()
                    ((:lambda (lst)
                       (:if (core:null lst)
                            ()
                            (mu:fx-sub (mu:length seq) (mu:length lst))))
                     (mu:fix
                      (:lambda (list)
                        (:if (core:null list)
                             ()
                             (:if (fn item (mu:car list))
                                  list
                                  (mu:cdr list))))
                      seq)))
               (:if (core:zerop (core:length seq))
                    ()
                    ((:lambda (nth)
                       (:if (core:null nth)
                            ()
                            nth))
                     (mu:fix
                      (:lambda (nth)
                        (:if (core:null nth)
                             ()
                             (:if (mu:fx-lt nth (mu:sv-len seq))
                                  (:if (fn item (mu:sv-ref seq nth))
                                       nth
                                       (core:1+ nth))
                                  ())))
                      0)))))))

(mu:intern core::ns :extern "positionr"
   (:lambda (fn item seq)
     (core:errorp-unless core:functionp fn "core:positionr: not a function")           
     (core:errorp-unless core:sequencep seq "core:positionr: not a sequence")
     (:if (core::fn-applyp fn)
          (:if (core:listp seq)
               (:if (core:null seq)
                    ()
                    ((:lambda (nthcdr)
                       (:if (core:null nthcdr)
                            ()
                            (mu:fx-sub (mu:length seq) (mu:length nthcdr))))
                     (mu:fix
                      (:lambda (nth)
                        (:if (core:listp nth)
                             nth
                             (:if (mu:fx-lt nth 0)
                                  ()
                                  (:if (core::apply fn (core::list2 item (mu:nth nth seq)))
                                       (mu:nthcdr nth seq)
                                       (core:1- nth)))))
                      (core:1- (mu:length seq)))))
               (:if (core:zerop (core:length seq))
                    ()
                    (mu:fix
                     (:lambda (nth)
                       (:if (core:null nth)
                            nth
                            (:if (mu:fx-lt nth 0)
                                 ()
                                 (:if (core::apply fn (core::list2 item (mu:sv-ref seq nth)))
                                      nth
                                      (core:1- nth)))))
                     (core:1- (mu:sv-len seq)))))
          (:if (core:listp seq)
               (:if (core:null seq)
                    ()
                    ((:lambda (nthcdr)
                       (:if (core:null nthcdr)
                            ()
                            (mu:fx-sub (mu:length seq) (mu:length nthcdr))))
                     (mu:fix
                      (:lambda (nth)
                        (:if (core:listp nth)
                             nth
                             (:if (mu:fx-lt nth 0)
                                  ()
                                  (:if (fn item (mu:nth nth seq))
                                       (mu:nthcdr nth seq)
                                       (core:1- nth)))))
                      (core:1- (mu:length seq)))))
               (:if (core:null seq)
                    ()
                    (mu:fix
                     (:lambda (nth)
                       (:if (core:null nth)
                            nth
                            (:if (mu:fx-lt nth 0)
                                 ()
                                 (:if (fn item (mu:sv-ref seq nth))
                                      nth
                                      (core:1- nth)))))
                     (core:1- (mu:sv-len seq))))))))
