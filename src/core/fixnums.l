;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fixnums
;;;
(mu::intern core::ns :extern "1+"
   (:lambda (n)
     (core:errorp-unless core:fixnump n "core:1+: not a fixnum")
     (mu:fixnum+ n 1)))

(mu::intern core::ns :extern "1-"
   (:lambda (n)
     (core:errorp-unless core:fixnump n "core:1-: not a fixnum")
     (mu:fixnum- n 1)))

(mu::intern core::ns :extern "truncate"
   (:lambda (n m)
     (core:errorp-unless core:fixnump n "core:mod: not a fixnum")
     (core:errorp-unless core:fixnump m "core:mod: not a fixnum")
     (mu:cons (mu:fixnum/ n m) (mu:fixnum- n (mu:fixnum* m (mu:fixnum/ n m))))))

;;; round to negative infinity
(mu::intern core::ns :extern "floor"
   (:lambda (n m)
     (core:errorp-unless core:fixnump n "core:rem: not a fixnum")
     (core:errorp-unless core:fixnump m "core:rem: not a fixnum")
     (mu:cons (mu:fixnum/ n m) (mu:fixnum- n (mu:fixnum* m (mu:fixnum/ n m))))))

;;; round to positive infinity
(mu::intern core::ns :extern "ceiling"
   (:lambda (n m)
     (core:errorp-unless core:fixnump n "core:mod: not a fixnum")
     (core:errorp-unless core:fixnump m "core:mod: not a fixnum")
     (mu:cons (mu:fixnum/ n m) (mu:fixnum- n (mu:fixnum* m (mu:fixnum/ n m))))))

(mu::intern core::ns :extern "mod"
   (:lambda (n m)
     (core:errorp-unless core:fixnump n "core:mod: not a fixnum")
     (core:errorp-unless core:fixnump m "core:mod: not a fixnum")
     (mu:cd (core:floor m n))))

(mu::intern core::ns :extern "rem"
   (:lambda (n m)
     (core:errorp-unless core:fixnump n "core:rem: not a fixnum")
     (core:errorp-unless core:fixnump m "core:rem: not a fixnum")
     (mu:cd (core:truncate m n))))

;;; round to nearest
(mu::intern core::ns :extern "round"
   (:lambda (n m)
     (core:errorp-unless core:fixnump n "core:rem: not a fixnum")
     (core:errorp-unless core:fixnump m "core:rem: not a fixnum")
     (mu:fixnum- (mu:fixnum/ n m) (mu:fixnum- n (mu:fixnum* m (mu:fixnum/ n m))))))

;;; ash
(mu::intern core::ns :extern "ash"
    (:lambda (fix count)
      (core:errorp-unless core:fixnump fix "core:ash: not a fixnum")
      (core:errorp-unless core:fixnump count "core:ash: not a fixnum")
      (:if (core:zerop count)
           fix
           (:if (mu:fixnum< count 0)
                (mu::fix*
                 (:lambda (self count fix)
                   (:if (core:zerop count)
                       fix
                       (core::prog
                          (env:fp-setv (core::fn-frame-id self) 1 (core:1- count))
                          (env:fp-setv (core::fn-frame-id self) 2 (mu:fixnum/ fix 2)))))
                 (core::list2 (mu:fixnum* -1 count) fix))
                (mu::fix*
                 (:lambda (self count fix)
                   (:if (core:zerop count)
                       fix
                       (core::prog
                          (env:fp-setv (core::fn-frame-id self) 1 (core:1- count))
                          (env:fp-setv (core::fn-frame-id self) 2 (mu:fixnum* fix 2)))))
                 (core::list2 count fix))))))