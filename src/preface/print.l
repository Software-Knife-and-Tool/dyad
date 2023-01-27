;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; common printing
;;;
(defun prin1 (form stream)
   (write form stream t))

(defun princ (form stream)
   (write form stream ()))

(defun pprint (form stream)
  (let ((pprint-threshold 3)
        (pprint-indents
         '((defun    2 2)
           (defmacro 2 2)
           (funcall  1 4)
           (if       1 4)
           (lambda   1 8)
           (cond     1 6)
           (let      1 4)
           (let*     1 4)
           (flet     1 4)
           (flet*    1 4)
           (list     1 1))))
    (flet* ((spaces (n stream)
              (errorp-unless fixnump n "")
              (errorp-unless streamp stream "")
              (flet ((loop (loop n)
                      (if (zerop n)
                          ()
                          ((lambda ()
                             (write " " stream ())
                             (loop loop (1- n)))))))
                (loop loop n)))
           (loop (loop form indent)
              (spaces indent stream)
             (cond
               ((consp form)
                (cond
                  ((symbolp (car form))
                   (let ((indent-desc (cdr (assoc (car form) pprint-indents))))
                     (if indent-desc
                         (cond
                           ((eq (nth 0 indent-desc) 1) 
                            (format stream "(~A ~A~%" (nth 0 form) (nth 1 form))
                            (mapc (lambda (el) (loop loop el (fixnum+ indent (nth 1 indent-desc)))) (nthcdr 2 form))
                            (format stream ")"))
                           ((eq (nth 0 indent-desc) 2) 
                            (format stream "(~A ~A ~A~%" (nth 0 form) (nth 1 form) (nth 2 form))
                            (mapc (lambda (el) (loop loop el (fixnum+ indent (nth 1 indent-desc)))) (nthcdr 3 form))
                            (format stream ")"))
                           (t (error form "pprint botch")))
                         (progn
                           (format stream "(")
                           (mapc (lambda (el) (loop loop el 1)) form)
                           (format stream ")")))))
                  (t (format stream "~A~%" form))))
               (t (format stream "~A" form)))))
            (loop loop form 0))))

