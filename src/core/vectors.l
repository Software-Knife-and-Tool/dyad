;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  core vectors
;;;
(mu:intern core::ns :extern "vector-type"
  (:lambda (vector)
    (core:raise-unless core:vectorp vector "core:vector-type: not a vector")

    (mu:sv-type vector)))

(mu:intern core::ns :extern "svref"
  (:lambda (vector nth)
    (core:raise-unless core:vectorp vector "core:svref: not a vector")
    (core:raise-unless core:fixnump nth "core:svref: not a fixnum")

    (mu:sv-ref vector nth)))

(mu:intern core::ns :intern "mapv"
  (:lambda (fn vector)
    (mu:fix
     (:lambda (nth)
       (:if nth
            (:if (mu:eq nth (mu:sv-len vector))
                 ()
                 (core::prog2
                     (core:apply fn (core::list (mu:sv-ref vector nth)))
                     (core:1+ nth)))
            ()))
     0)))
