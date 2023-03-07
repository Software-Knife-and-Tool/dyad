;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; internal utilities
;;;
(mu:intern core::ns :intern "append"
   (:lambda (list-1 list-2)
      (core:foldr
        mu:cons
        list-2
        list-1)))

(mu:intern core::ns :intern "list"
   (:lambda (form)
     (mu:cons form ())))

(mu:intern core::ns :intern "list2"
   (:lambda (f1 f2)
     (mu:cons f1 (core::list f2))))

(mu:intern core::ns :intern "list3"
   (:lambda (f1 f2 f3)
     (mu:cons f1 (core::list2 f2 f3))))

;;;
;;; fixpoint loop
;;;
(mu:intern core::ns :extern "fix"
   (:lambda (fn list)
     (core:raise-unless core:functionp fn "core:fix: not a function")
     (core:raise-unless core:listp list "core:fix: not a list")
     (:if (core::fn-applyp fn)
          (mu:fix (:lambda (arg) (core:apply fn (core::list arg))) list)
          (mu:fix fn list))))

;;;
;;; folds
;;;
(mu:intern core::ns :extern "foldl"
  (:lambda (fn init list)
      (core:raise-unless core:functionp fn "core:foldl: not a function")
      (core:raise-unless core:listp list "core:foldl: not a list")
      (mu:cdr
       (mu:fix
        (:lambda (arg)
          ((:lambda (list acc)
             (:if list
                  (mu:cons
                   (mu:cdr list)
                   (core:apply fn (core::list2 (mu:car list) acc)))
                  arg))
           (mu:car arg)
           (mu:cdr arg)))
        (mu:cons list init)))))

(mu:intern core::ns :extern "foldr"
  (:lambda (fn init list)
    (core:raise-unless core:functionp fn "core:foldr: not a function")
    (core:raise-unless core:listp list "core:foldr: not a list")
    (core:foldl fn init (core:reverse list))))

;;;
;;; maps
;;;
(mu:intern core::ns :extern "mapc"
   (:lambda (fn list)
     (core:raise-unless core:functionp fn "core:mapc: not a function")
     (core:raise-unless core:listp list "core:mapc: not a list")
     (mu:fix
      (:lambda (lst)
        (:if lst
             (core::prog2
                 (core:apply fn (core::list (mu:car lst)))
                 (mu:cdr lst))
             ()))
      list)
     list))

(mu:intern core::ns :extern "mapcar"
   (:lambda (fn list)
     (core:raise-unless core:functionp fn "core:mapcar: not a function")
     (core:raise-unless core:listp list "core:mapcar: not a list")
     (core:foldl
      (:lambda (elt acc)
        (core::append acc (core::list (core:apply fn (core::list elt)))))
      ()
      list)))
   
(mu:intern core::ns :extern "mapl"
   (:lambda (fn list)
      (core:raise-unless core:functionp fn "core:mapl: not a function")
      (core:raise-unless core:listp list "core:mapl: not a list")
      (mu:fix
       (:lambda (list)
         (:if list
              (core::prog2
                  (core:apply fn (core::list list))
                  (mu:cdr list))
              ()))
       list)
      list))

(mu:intern core::ns :extern "maplist"
   (:lambda (fn list)
     (core:raise-unless core:functionp fn "core:maplist: not a function")
     (core:raise-unless core:listp list "core:maplist: not a list")
     (mu:car
      (mu:fix
       (:lambda (loop)
         ((:lambda (acc list)
            (:if list
                 (mu:cons
                  (core::append acc (core::list (core:apply fn (core::list list))))
                  (mu:cdr list))
                 loop))
          (mu:car loop)
          (mu:cdr loop)))
       (mu:cons () list)))))

;;;
;;; finds
;;;
(mu:intern core::ns :extern "findl"
   (:lambda (fn list)
     (core:raise-unless core:functionp fn "core:findl: not a function")
     (core:raise-unless core:listp list "core:findl: not a list")
     (core:foldr
      (:lambda (el acc)
        (:if (core:apply fn (core::list el))
             el
             acc))
      ()
      list)))

(mu:intern core::ns :extern "findr"
   (:lambda (fn list)
     (core:raise-unless core:functionp fn "core:findr not a function")
     (core:raise-unless core:sequencep list "core:findr not a list")
     (core:foldl
      (:lambda (el acc)
        (:if (core:apply fn (core::list el))
             el
             acc))
      ()
      list)))

;;;
;;; position
;;;
(mu:intern core::ns :extern "positionl"
   (:lambda (fn list)
     (core:raise-unless core:functionp fn "core:positionl: not a function")           
     (core:raise-unless core:listp list "core:positionl: not a list")
     (:if (core:null list)
          ()
          ((:lambda (len)
             (mu:car
              (mu:fix
               (:lambda (loop)
                 (:if (core:listp loop)
                      loop
                      (:if (mu:fx-lt len loop)
                           ()
                           (:if (core:apply fn (core::list (mu:nth loop list)))
                                (core::list loop)
                                (core:1+ loop)))))
               0)))
           (mu:length list)))))

(mu:intern core::ns :extern "positionr"
   (:lambda (fn list)
     (core:raise-unless core:functionp fn "core:positionr: not a function")           
     (core:raise-unless core:listp list "core:positionr: not a list")
     (:if (core:null list)
          ()
          (mu:car
           (mu:fix
            (:lambda (loop)
              (:if (core:listp loop)
                   loop
                   (:if (mu:fx-lt loop 0)
                        ()
                        (:if (core:apply fn (core::list (mu:nth loop list)))
                             (core::list loop)
                             (core:1- loop)))))
            (core:1- (mu:length list)))))))

;;;
;;; fixed argument list functions
;;;
(mu:intern core::ns :extern "append"
   (:lambda (list)
     (core:raise-unless core:listp list "core:append: not a list")
     (core:foldr
      (:lambda (el acc)
        (:if (core:listp el)
          (core:foldr mu:cons acc el)
          el))
      ()
      list)))

(mu:intern core::ns :extern "reverse"
  (:lambda (list)
    (core:raise-unless core:listp list "core:reverse: not a list")
    (core:foldl mu:cons () list)))

(mu:intern core::ns :extern "dropl"
   (:lambda (list n)
      (core:raise-unless core:listp list "core:dropl: not a list")
      (core:raise-unless core:fixnump n "core:dropl: not a fixnum")
      (mu:nthcdr n list)))

(mu:intern core::ns :extern "dropr"
   (:lambda (list n)
      (core:raise-unless core:listp list "core:dropr: not a list")
      (core:raise-unless core:fixnump n "core:dropr: not a fixnum")
      (core:foldl
        mu:cons
        ()
        ((:lambda (len)
            (core:foldl
              (:lambda (el last)
                 (:if (mu:fx-lt (mu:length last) (mu:fx-sub (mu:length list) n))
                      (mu:cons el last)
                      last))
                 ()
              list))
         (mu:length list)))))

(mu:intern core::ns :extern "assoc"
   (:lambda (item list)
     (mu:car
        (mu:fix
         (:lambda (lst)
           (:if lst
               ((:lambda (entry)
                  (core:raise-unless core:consp entry "core:assoc: not a cons")
                  (:if (mu:eq item (mu:car entry))
                      lst
                      (mu:cdr lst)))
                (mu:car lst))
               ()))
         list))))

(mu:intern core::ns :extern "last"
   (:lambda (list)
     (core:raise-unless core:listp list "core:last: not a list")
     (:if list
          (mu:nthcdr (core:1- (mu:length list)) list)
          ())))
