;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  core symbols
;;;
(mu:intern core::ns :extern "symbol-name" mu:sy-name)
(mu:intern core::ns :extern "symbol-ns" mu:sy-ns)
(mu:intern core::ns :extern "symbol-value" mu:sy-val)

(mu:intern core::ns :intern "intern"
  (:lambda (symbol value env)
    (:if (core:uninternedp symbol)
         ((:lambda (sym)
            (:if sym
                 (:if (core:boundp sym)
                      (core:raise sym "intern: symbol already bound")
                      (mu:intern (core::read-get-ns) :extern (mu:sy-name symbol)value))
                 (mu:intern (core::read-get-ns) :extern (mu:sy-name symbol) value)))
          (mu:ns-map (core::read-get-ns) :extern (mu:sy-name symbol)))
         ((:lambda (ns name)
             (mu:intern
              (core::read-get-ns)
              (:if (mu:ns-map ns :extern name)
                   :extern
                   :intern)
              name
              value))
           (mu:sy-ns symbol)
          (mu:sy-name symbol)))))
