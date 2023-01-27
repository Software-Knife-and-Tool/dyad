---
title: print library
---

@comment_type ;;; %s
@title Appendix - System Describe

@s System Describe
<hr><br/>
<p></p>

--- system describe --- noWeave
(defun system::describe-function (fn)
  (assert functionp fn "system:describe-function requires a function")
  (let ((desc (core::fn-lambda fn)))
    (system:format t "   name: ~A~%" (core::fn-name fn))
    (system:format t "   nreq: ~A~%" (core::fn-nreqs fn))
    (system:format t " lambda: ~A~%" (core::fn-form fn))
    (cond
      ((null desc) (system:format t "has no lambda extension~%"))
      (t
        (system:format t "closure: ~A~%" (core::lambda-closure desc))
        (system:format t "    env: ~A~%" (core::lambda-env desc))
        (system:format t " macrop: ~A~%" (core::lambda-macrop desc))
        (system:format t "   reqs: ~A~%" (core::lambda-reqs desc))
        (system:format t "   rest: ~A~%" (core::lambda-rest desc))))
  fn))

(defun system::describe-symbol (symbol)
  (assert symbolp symbol "system:describe-symbol requires a symbol")
    (system:format t "is a keyword: ~A~%" (mu:keyp symbol))
    (system:format t "   name: ~a~%" (mu:sy-name symbol))
    (system:format t "     ns: ~A~%" (mu:sy-ns symbol))
    (if (mu:boundp symbol)
        (system:format t "  value: ~A~%" (mu:sy-val symbol))
        (system:format t "  value: is unbound~%"))
  symbol)

(defun system::describe-vector (vector)
  (assert vectorp vector "system:describe-vector requires a vector")
    (system:format t "  type: ~A~%" (mu:sv-type vector))
    (system:format t "length: ~A~%" (mu:sv-len vector))
  vector)

(defun system:describe (obj)
  (cond
    ((consp obj) (system:format t "is a cons: length ~A ~A~%" (length obj) obj))
    ((functionp obj) (system:format t "is a function: ~A~%" obj) (system::describe-function obj))
    ((fixnump obj) (system:format t "is a fixnum: ~A~%" obj))
    ((stringp obj) (system:format t "is a string byte vector: ~a~%" obj))
    ((symbolp obj) (system:format t "is a symbol: ~A~%" obj) (system::describe-symbol obj))
    ((vectorp obj) (system
    :format t "is a vector: ~A~%" obj) (system::describe-vector obj))
    (t (system:format t "is undescribed: ~A~%" obj))))
---

--- system inspect --- noWeave
(defun system:inspect (object)
  ((lambda (ifs)
     (check-type ifs :stream "system:inspect cannot open input stream")
     (system:format t ":h for commands~%")
     (system:format t "inspecting ~A~%" object)
     ((lambda (loop)
        (with-ex
          (lambda (ex)
            (print-except ex "system:inspect")
            (break ex))
          (lambda ()
            (loop loop (list object)))))
      (lambda (loop stack)
        (system:format t "inspect> ")
        (let ((cmd (core:read ifs t)))
          (if (eofp ifs)
              ()
              (progn
                (if (keyp cmd)
                    (cond
                      ((eq cmd :h) (system:format t "inspector:~%")
                                   (system:format t "  :v - view~%")
                                   (system:format t "  :d - describe~%")
                                   (system:format t "  :[0..n] - inspect view index~%")
                                   (system:format t "  :n - inspect vector/list index~%")
                                   (system:format t "  :p - print~%")
                                   (system:format t "  :s - stack~%")
                                   (system:format t "  :x - exit~%")
                                   (system:format t "  :t - type~%")
                                   (system:format t "  :r - pop~%"))
                      ((eq cmd :d) (system:format t "~A " (car stack)) (system:describe (car stack)))
                      ((eq cmd :i) (loop loop (list* (core:eval (core:compile (core:read ifs t) ())) stack)))
                      ((eq cmd :p) (system:format t "~A~%" (car stack)))
                      ((eq cmd :s) (system:format t "~A~%" stack))
                      ((eq cmd :t) (system:format t "~A~%" (type-of (car stack))))
                      ((eq cmd :v) (system:format t "~A~%" (mu::view (car stack))))
                      ((eq cmd :x))
                      (t (system:format t "~A~%" cmd)))
                   (system:format t "~A~%" form))
                (loop loop stack)))))))
     std-in))
---

--- system-describe --- noWeave
@{system describe}
@{system inspect}
---

--- system:describe.l --- noWeave
@{system-describe}
---
