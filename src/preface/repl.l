;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; inspector
;;;
(defun inspect (object)
  ((lambda (ifs)
     (errorp-unless streamp ifs "inspect: cannot open input stream")
     (format t ":h for commands~%")
     (format t "inspecting ~A~%" object)
     ((lambda (loop)
        (with-ex
          (lambda (ex)
            (print-except ex "inspect")
            (break ex))
          (lambda ()
            (loop loop (list object)))))
      (lambda (loop stack)
        (format t "inspect> ")
        (let ((cmd (read ifs t)))
          (if (eofp ifs)
              ()
              (progn
                (if (keywordp cmd)
                    (cond
                      ((eq cmd :h) (format t "inspector:~%")
                                   (format t "  :v - view~%")
                                   (format t "  :d - describe~%")
                                   (format t "  :[0..n] - inspect view index~%")
                                   (format t "  :n - inspect vector/list index~%")
                                   (format t "  :p - print~%")
                                   (format t "  :s - stack~%")
                                   (format t "  :x - exit~%")
                                   (format t "  :t - type~%")
                                   (format t "  :r - pop~%"))
                      ((eq cmd :d) (format t "~A " (car stack)) (describe (car stack)))
                      ((eq cmd :i) (loop loop (list* (eval (compile (read ifs t) ())) stack)))
                      ((eq cmd :p) (format t "~A~%" (car stack)))
                      ((eq cmd :s) (format t "~A~%" stack))
                      ((eq cmd :t) (format t "~A~%" (type-of (car stack))))
                      ((eq cmd :v) (format t "~A~%" (mu::view (car stack))))
                      ((eq cmd :x))
                      (t (format t "~A~%" cmd)))
                   (format t "~A~%" form))
                (loop loop stack)))))))
     std-in))

;;;
;;; preface repl
;;;
(defun repl (ns)
  (errorp-unless namespacep ns "repl: not a namespace")
  (flet ((repl (repl)
          (with-ex
              (lambda (ex)  ;; catch exception and break
                (print-except ex "preface:repl")
                (break ex)
                (repl repl))
              (lambda ()    ;; main
                (flet ((loop (loop form)
                        (if (eofp t)
                            (exit 0)
                            (if (keywordp form)
                                (cond
                                  ((eq form :h)
                                   (format t ";;; help: libmu ~A~%:l - load file~%:m - expand macro~%" mu::version))
                                  ((eq form :m) (format t "~A~%" (macroexpand-1 (read t t ()))))
                                  ((eq form :l)
                                   ((lambda (path)
                                      (if (stringp path)
                                          (load path t ())
                                          (format t ":l ~A is not a file path~%" path)))
                                    (read t t ())))
                                  (t ((lambda ()
                                        (write form () ())
                                        (terpri ())))))
                                  (format t "~A~%" (eval (compile form)))))
                        (format t "~A> " (ns-name ns))
                        (loop loop (read t t ()))))
                  (format t "~A> " (ns-name ns))
                  (loop loop (read t t ())))))))
    (format t ";;; Eko Lisp ~A (preface:repl) :h for help~%" preface::eko-version)
    (repl repl)))
