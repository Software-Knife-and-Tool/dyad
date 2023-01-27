;;;
;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  dyad runtime environment
;;;
(core:load "/opt/dyad/dyad/preface/preface.l" () ())

(mu:eval (core:compile '(defconst preface::require-path "/opt/dyad/lib")))

(core:in-namespace (mu:ns "user" (env:find-ns "preface")))

(:if (core:zerop (system:system "stat ~/.dyad > /dev/null 2>&1"))
     (core:load "~/.dyad" () ())
     ())

(core:mapc
 (:lambda (fn) (core:load fn :t ()))
 (mu:read (mu:open :buf :input (system:getenv "DYAD_LOAD_LIST"))))

(mu::intern (env:find-ns "preface") :intern "dyad-version" "0.0.1")

(preface:repl (env:find-ns "user"))
