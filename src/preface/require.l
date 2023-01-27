;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; preface require
;;;
(defconst preface::require-ns (mu:ns (symbol-name (gensym)) (env:find-ns "preface")))
(core:in-namespace preface::require-ns)

(defmacro preface:require-lib (lib-sym)
  (errorp-unless symbolp lib-sym "require-lib: not a symbol")
  (let ((lib (symbol-name lib-sym)))
    (if (mu:ns-find preface::require-ns :extern lib)
      ()
      (let ((lib-path (core:format () "~A/~A:~A.l" (list preface::require-path "eko" lib))))
        (mu:intern preface::require-ns :extern lib)
        (core:load lib-path () ())))))

(defmacro preface:require-lib-verbose (lib-sym)
  (errorp-unless symbolp lib-sym "require-lib not a symbol")
  (let ((lib (symbol-name lib-sym)))
    (if (mu:ns-find preface::require-ns :extern lib)
      ()
      (let ((lib-path (core:format () "~A/~A:~A.l" (list preface::require-path "eko" lib))))
        (mu:intern preface::require-ns :extern lib)
        (core:load lib-path :t ())))))

