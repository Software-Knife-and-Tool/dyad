---
title: print library
---

(defun common:prin1 (form stream)
   (mu:print form stream t))

(defun common:princ (form stream)
   (mu:print form stream ()))

(defun common:pprint (form stream)
  (let ((pprint-threshold 4)
        (pprint-indents
         '((defun    2 2)
           (defmacro 2 2)
           (funcall  1 4)
           (if       1 4)
           (lambda   1 8)
           (cond     1 6)
           (let      1 4)
           (let*     1 4)
           (list     1 1)))
        (stream (core::stream-designator stream)))
    ((lambda (loop) (loop loop form 0))
     (lambda (loop form indent)
       (common:dotimes (_ indent) (system:format stream " "))
       (cond
         ((consp form)
          (cond
            ((symbolp (car form))
               (let ((indent-desc (assoc (car form) pprint-indents)))
                 (if indent-desc
                     (cond
                       ((eq (nth 0 indent-desc) 1) 
                          (system:format stream "(~A ~A~%" (nth 0 form) (nth 1 form))
                          (mapc (lambda (el) (loop loop el (fixnum+ indent (nth 1 indent-desc)))) (nthcdr 2 form))
                          (system:format stream ")"))
                       ((eq (nth 0 indent-desc) 2) 
                          (system:format stream "(~A ~A ~A~%" (nth 0 form) (nth 1 form) (nth 2 form))
                          (mapc (lambda (el) (loop loop el (fixnum+ indent (nth 1 indent-desc)))) (nthcdr 3 form))
                          (system:format stream ")"))
                       (t (core:warn form "ix:pprint botch")))
                     (progn
                       (system:format stream "(")
                       (mapc (lambda (el) (loop loop el 1)) form)
                       (system:format stream ")")))))))
          (t (system:format stream "~A" form)))))
      ()))
