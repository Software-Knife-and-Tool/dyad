;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fixpoint loops
;;;
(mu:intern core::ns :extern "fix"
   (:lambda (fn list)
     (core:errorp-unless core:functionp fn "core:fix: not a function")
     (core:errorp-unless core:listp list "core:fix: not a list")
     (:if (core::fn-funcallp fn)
          (mu:fix (:lambda (arg) (core::funcall fn (core::list arg))) list)
          (mu:fix fn list))))

(mu:intern core::ns :extern "fix*"
   (:lambda (fn list)
     (core:errorp-unless core:functionp fn "core:fix*: not a function")
     (core:errorp-unless core:listp list "core:fix*: not a list")
     (:if (core::fn-funcallp fn)
          (mu:fix* (:lambda (arg) (core::funcall fn (core::list arg))) list)
          (mu:fix* fn list))))

;;;
;;; lists
;;;
(mu:intern core::ns :intern "list"
   (:lambda (form)
     (mu:cons form ())))

(mu:intern core::ns :intern "list2"
   (:lambda (f1 f2)
     (mu:cons f1 (core::list f2))))

(mu:intern core::ns :intern "list3"
   (:lambda (f1 f2 f3)
     (mu:cons f1 (core::list2 f2 f3))))

(mu:intern core::ns :extern "append"
   (:lambda (list)
     (core:errorp-unless core:listp list "core:append: not a list")
     (core:foldr
      (:lambda (el acc)
        (:if (core:listp el)
          (core:foldr (:lambda (el acc) (mu:cons el acc)) acc el)
          el))
      ()
      list)))

(mu:intern core::ns :extern "mapc"
   (:lambda (fn list)
     (core:errorp-unless core:functionp fn "core:mapc: not a function")
     (core:errorp-unless core:listp list "core:mapc: not a list")
     (:if (core::fn-funcallp fn)
          (mu:fix
           (:lambda (list)
             (:if (mu:eq () list)
                  ()
                  (core::prog2
                     (core::funcall fn (core::list (mu:car list)))
                     (mu:cdr list))))
           list)
          (mu:fix
           (:lambda (list)
             (:if (mu:eq () list)
                  ()
                  (core::prog2 (fn (mu:car list)) (mu:cdr list))))
           list))
     list))

(mu:intern core::ns :extern "mapcar"
   (:lambda (fn list)
     (core:errorp-unless core:functionp fn "core:mapcar: not a function")
     (core:errorp-unless core:listp list "core:mapcar: not a list")
     (:if (core::fn-funcallp fn)
          (core:foldl
           (:lambda (elt acc)
             (core::append acc (core::list (fn elt))))
           ()
           list)
          (core:foldl
           (:lambda (elt acc)
             (core::append acc (core::list (core::funcall fn (core::list elt)))))
           ()
           list))))
   
(mu:intern core::ns :extern "mapl"
   (:lambda (fn list)
      (core:errorp-unless core:functionp fn "core:mapl: not a function")
      (core:errorp-unless core:listp list "core:mapl: not a list")
      (:if (core::fn-funcallp fn)
           (mu:fix
            (:lambda (list)
              (:if (mu:eq () list)
                   ()
                   (core::prog2
                      (core::funcall fn (core::list list))
                      (mu:cdr list))))
            list)
           (mu:fix
            (:lambda (list)
              (:if (mu:eq () list)
                   ()
                   (core::prog2 (fn list) (mu:cdr list))))
            list))
      list))

(mu:intern core::ns :extern "maplist"
   (:lambda (fn list)
     (core:errorp-unless core:functionp fn "core:maplist+: not a function")
     (core:errorp-unless core:listp list "core:maplist+: not a list")
     (:if (core:null list)
          ()
          (:if (core::fn-funcallp fn)
               (mu:fix*
                (:lambda (self acc list)
                  (:if (core:null list)
                       acc
                       (core:prog
                          (mu:fr-setv (core::fn-frame-id self)
                                       1
                                       (core::append acc (core::list (core::funcall fn (core::list list)))))
                          (mu:fr-setv (core::fn-frame-id self) 2 (mu:cdr list)))))
                (core::list2 () list))
               (mu:fix*
                (:lambda (self acc list)
                  (:if (core:null list)
                       acc
                       (core::prog2
                          (mu:fr-setv (core::fn-frame-id self)
                                       1
                                       (core::append acc (core::list (fn list))))
                          (mu:fr-setv (core::fn-frame-id self) 2 (mu:cdr list)))))
                (core::list2 () list))))))

(mu:intern core::ns :intern "append"
   (:lambda (list-1 list-2)
      (core:foldr
        mu:cons
        list-2
        list-1)))

;;; generalize this to coerce?
(mu:intern core::ns :extern "sv-list"
   (:lambda (vec)
      (core:errorp-unless core:vectorp vec "core:sv-list not: a vector")
      (core:foldl
        mu:cons
        ()
        (core:foldl
          mu:cons
          ()
          vec))))

(mu:intern core::ns :extern "dropl"
   (:lambda (list n)
      (core:errorp-unless core:listp list "core:dropl: not a list")
      (core:errorp-unless core:fixnump n "core:dropl: not a fixnum")
      (mu:nthcdr n list)))

(mu:intern core::ns :extern "dropr"
   (:lambda (list n)
      (core:errorp-unless core:listp list "core:dropr: not a list")
      (core:errorp-unless core:fixnump n "core:dropr: not a fixnum")
      (core:foldl
        mu:cons
        ()
        ((:lambda (len)
            (core:foldl
              (:lambda (el last)
                 (:if (mu:fixnum< (mu:length last) (mu:fixnum- (mu:length list) n))
                      (mu:cons el last)
                      last))
                 ()
              list))
         (mu:length list)))))

(mu:intern core::ns :extern "assoc"
   (:lambda (item list)
     (mu:car
        (mu:fix
         (:lambda (list)
           (:if (mu:eq () list)
               ()
               ((:lambda (entry)
                  (core:errorp-unless core:consp entry "core:assoc: not a cons")
                  (:if (mu:eq item (mu:car entry))
                      list
                      (mu:cdr list)))
                (mu:car list))))
         list))))

(mu:intern core::ns :extern "last"
   (:lambda (list)
     (core:errorp-unless core:listp list "core:last: not a list")
     (:if list
          (mu:nthcdr (core:1- (mu:length list)) list)
          ())))
