;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  core vectors
;;;
(mu:intern core::ns :extern "vector-type"
   (:lambda (vector)
     (core:errorp-unless core:vectorp vector "core:vector-type: not a vector")
     (mu:sv-type vector)))

(mu:intern core::ns :extern "svref"
   (:lambda (vector nth)
     (core:errorp-unless core:vectorp vector "core:svref: not a vector")
     (core:errorp-unless core:fixnump nth "core:svref: not a fixnum")
     (mu:sv-ref vector nth)))
