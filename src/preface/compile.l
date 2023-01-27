;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT
(defun compile-form (form)
  (format t "compile-file: ~A~%" form)
  (flet ((parse (form)
                (if (consp form)
                    (mapcar preface:compile-form form)
                    form))
         (const (form)
                (cond
                  ((and (symbolp form)
                        (boundp form)
                        (not (symbolp (symbol-value form)))
                        (not (consp (symbol-value form))))
                   (preface:format t ";;; constant propagation: ~A ~A~%" form (symbol-value form))
                   (symbol-value form))
                  (t form)))
         (review (form)
                 form))
        (foldl
         (lambda (fn intermediate)
           (funcall fn intermediate))
         form
         (list parse const))))

;;;
;;; compile-file
;;;
(defun compile-file (input-path output-path &rest options)
  (errorp-unless stringp input-path "compile-file: not a pathname")
  (errorp-unless stringp output-path "compile-file: not a pathname")
  (with-open-file out output-path :output
    (with-open-file src input-path :input
      (do* ((form () (read src () ())))
           ((eofp src) t)
        (format t ":read ~A~%" form)
        (format out ";;; ~A~%" form)))))
