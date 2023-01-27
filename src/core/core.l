;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  core namespace
;;;
(env:ns (mu:ns "core" (env:find-ns "mu")))
(mu::intern (env:find-ns "core") :intern "ns" (env:find-ns "core"))

;;;
;;; predicates
;;;
(mu::intern core::ns :extern "charp" (:lambda (t) (mu:eq :char (mu:type-of t))))
(mu::intern core::ns :extern "consp" (:lambda (t) (mu:eq :cons (mu:type-of t))))
(mu::intern core::ns :extern "doublep" (:lambda (t) (mu:eq :double (mu:type-of t))))
(mu::intern core::ns :extern "exceptionp" (:lambda (t) (mu:eq :except (mu:type-of t))))
(mu::intern core::ns :extern "fixnump" (:lambda (t) (mu:eq :fixnum (mu:type-of t))))
(mu::intern core::ns :extern "floatp" (:lambda (t) (mu:eq :float (mu:type-of t))))
(mu::intern core::ns :extern "functionp" (:lambda (t) (mu:eq :func (mu:type-of t))))
(mu::intern core::ns :extern "namespacep" (:lambda (t) (mu:eq :ns (mu:type-of t))))
(mu::intern core::ns :extern "streamp" (:lambda (t) (mu:eq :stream (mu:type-of t))))
(mu::intern core::ns :extern "symbolp" (:lambda (t) (mu:eq :symbol (mu:type-of t))))
(mu::intern core::ns :extern "vectorp" (:lambda (t) (mu:eq :vector (mu:type-of t))))
(mu::intern core::ns :extern "listp" (:lambda (t) (:if (core:consp t) :t (core:null t))))
(mu::intern core::ns :extern "keywordp" (:lambda (t) (:if (mu:keysymp t) :t (core:null t))))
(mu::intern core::ns :extern "sequencep" (:lambda (t) (:if (core:listp t) :t (core:vectorp t))))

(mu::intern core::ns :extern "atom"
   (:lambda (t)
     (core:null (core:consp t))))

(mu::intern core::ns :extern "boundp" mu:boundp)
(mu::intern core::ns :extern "fboundp"
   (:lambda (symbol)
     (:if (mu:boundp symbol)
          (core:functionp (mu:sy-val symbol))
          ())))

(mu::intern core::ns :extern "uninternedp"
   (:lambda (sym)
     (core:null (mu:sy-ns sym))))

(mu::intern core::ns :extern "zerop"
   (:lambda (n)
     (:if (mu:eq 0 n)
          :t
          (mu:eq 0.0 n))))

(mu::intern core::ns :extern "numberp"
   (:lambda (n)
     (:if (core:fixnump n)
          :t
          (core:floatp n))))

;;;
;;; generalized booleans
;;;
(mu::intern core::ns :extern "null" (:lambda (t) (mu:eq :nil t)))
(mu::intern core::ns :extern "not" core:null)

(mu::intern core::ns :intern "logical-or"
    (:lambda (arg arg1)
      (:if arg
           arg
           arg1)))

(mu::intern core::ns :intern "logical-and"
    (:lambda (arg arg1)
      (:if arg
           arg1
           ())))

(mu::intern core::ns :intern "prog"
    (:lambda (arg1 arg2)
      arg2))

(mu::intern core::ns :intern "prog1"
    (:lambda (arg1 arg2)
      arg1))
