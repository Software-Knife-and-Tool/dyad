;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fixnums
;;;
(mu:intern core::ns :extern "1+"
   (:lambda (n)
     (core:raise-unless core:fixnump n "core:1+: not a fixnum")
     (mu:fx-add n 1)))

(mu:intern core::ns :extern "1-"
   (:lambda (n)
     (core:raise-unless core:fixnump n "core:1-: not a fixnum")
     (mu:fx-sub n 1)))

(mu:intern core::ns :extern "truncate"
   (:lambda (n m)
     (core:raise-unless core:fixnump n "core:mod: not a fixnum")
     (core:raise-unless core:fixnump m "core:mod: not a fixnum")
     (mu:cons (mu:fx-div n m) (mu:fx-sub n (mu:fx-mul m (mu:fx-div n m))))))

;;; round to negative infinity
(mu:intern core::ns :extern "floor"
   (:lambda (n m)
     (core:raise-unless core:fixnump n "core:rem: not a fixnum")
     (core:raise-unless core:fixnump m "core:rem: not a fixnum")
     (mu:cons (mu:fx-div n m) (mu:fx-sub n (mu:fx-mul m (mu:fx-div n m))))))

;;; round to positive infinity
(mu:intern core::ns :extern "ceiling"
   (:lambda (n m)
     (core:raise-unless core:fixnump n "core:mod: not a fixnum")
     (core:raise-unless core:fixnump m "core:mod: not a fixnum")
     (mu:cons (mu:fx-div n m) (mu:fx-sub n (mu:fx-mul m (mu:fx-div n m))))))

(mu:intern core::ns :extern "mod"
   (:lambda (n m)
     (core:raise-unless core:fixnump n "core:mod: not a fixnum")
     (core:raise-unless core:fixnump m "core:mod: not a fixnum")
     (mu:cd (core:floor m n))))

(mu:intern core::ns :extern "rem"
   (:lambda (n m)
     (core:raise-unless core:fixnump n "core:rem: not a fixnum")
     (core:raise-unless core:fixnump m "core:rem: not a fixnum")
     (mu:cd (core:truncate m n))))

;;; round to nearest
(mu:intern core::ns :extern "round"
   (:lambda (n m)
     (core:raise-unless core:fixnump n "core:rem: not a fixnum")
     (core:raise-unless core:fixnump m "core:rem: not a fixnum")
     (mu:fx-sub (mu:fx-div n m) (mu:fx-sub n (mu:fx-mul m (mu:fx-div n m))))))

;;; ash
(mu:intern core::ns :extern "ash"
    (:lambda (fix count)
      (core:raise-unless core:fixnump fix "core:ash: not a fixnum")
      (core:raise-unless core:fixnump count "core:ash: not a fixnum")
      (:if (core:zerop count)
           fix
           (:if (mu:fx-lt count 0)
                (mu:fix
                 (:lambda (self count fix)
                   (:if (core:zerop count)
                       fix
                       (core::prog2
                          (mu:fr-setv (core::fn-frame-id self) 1 (core:1- count))
                          (mu:fr-setv (core::fn-frame-id self) 2 (mu:fx-divXS fix 2)))))
                 (core::list2 (mu:fx-mul -1 count) fix))
                (mu:fix
                 (:lambda (self count fix)
                   (:if (core:zerop count)
                       fix
                       (core::prog2
                          (mu:fr-setv (core::fn-frame-id self) 1 (core:1- count))
                          (mu:fr-setv (core::fn-frame-id self) 2 (mu:fx-mul fix 2)))))
                 (core::list2 count fix))))))
