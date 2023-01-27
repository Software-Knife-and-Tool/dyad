;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; eql
;;;
(defun eql (x y)
   (cond
     ((eq x y) t)
     ((null (eq (type-of x) (type-of y))) nil)
     ((stringp x) (string= x y))
     ((and (vectorp x) (vectorp y))
        (and (eq (vector-type x) (vector-type y))
             (eq (vector-length x) (vector-length y))
             (core::funcall eql (vector-to-list x) (vector-to-list y))))
     ((consp x)
        (and (eq (length x) (length y))
             (core::funcall eql (car x) (car y))
             (core::funcall eql (cdr x) (cdr y))))
     (t ())))

;;;
;;; dotimes macro
;;;
(defmacro dotimes (ctr-list &rest body)
  (errorp-unless consp ctr-list "dotimes: not an init list")
  (error-if (fixnum< (length ctr-list) 2) ctr-list "dotimes: malformed init list")
  (error-if (fixnum< 3 (length ctr-list)) ctr-list "dotimes: malformed init list")
  (let ((count-var  (nth 0 ctr-list))
        (count-form (nth 1 ctr-list))
        (result-form (nth 2 ctr-list))
        (limit-gsym (gensym)))
    (errorp-unless symbolp count-var "dotimes: not a symbol")
    (list (list 'lambda (list limit-gsym)
                (list 'if (list 'fixnum< limit-gsym 0)
                      ()
                      (list 'mu::fix
                            (list 'lambda (list count-var)
                                  (list 'if (list 'eq count-var limit-gsym)
                                        count-var
                                        (list* 'progn (append body (list (list '1+ count-var))))))
                            0))
                result-form)
          count-form)))

;;;
;;; dolist macro
;;;
(defmacro dolist (init-list &rest body)
  (errorp-unless consp init-list "dolist; not an init list")
  (let ((init (nth 1 init-list))
        (sym (car init-list)))
    (errorp-unless symbolp sym "dolist; not a symbol")  
    (errorp-unless listp init "dolist; not a list")
    (list
      (list 'lambda ()
        (list
         'mapc
         (list* 'lambda (list sym) body)
         (list 'eval init))
        ()))))

;;;
;;; with-open-file macro
;;;
(defmacro with-open-file (symbol path dir &rest body)
  (list 'let (list (list symbol (list core:open-file dir path)))
    (list 'with-ex
      (list 'lambda (list 'ex)
        (list 'print-except 'ex "with-open-file")
        (list 'close symbol))
      (list* 'lambda () (append body (list (list 'close symbol)))))))

;;;
;;; cxxxxr
;;;
(defun caar (x) (car (car x)))
(defun cadr (x) (car (cdr x)))   
(defun cdar (x) (cdr (car x)))   
(defun cddr (x) (cdr (cdr x)))   
(defun caaar (x) (car (car (car x))))  
(defun caadr (x) (car (car (cdr x))))  
(defun cadar (x) (car (cdr (car x))))  
(defun caddr (x) (car (cdr (cdr x))))  
(defun cdaar (x) (cdr (car (car x))))  
(defun cdadr (x) (cdr (car (cdr x))))  
(defun cddar (x) (cdr (cdr (car x))))  
(defun cdddr (x) (cdr (cdr (cdr x))))  
(defun caaaar (x) (car (car (car (car x)))))
(defun caaadr (x) (car (car (car (cdr x))))) 
(defun caadar (x) (car (car (cdr (car x))))) 
(defun caaddr (x) (car (car (cdr (cdr x))))) 
(defun cadaar (x) (car (cdr (car (car x))))) 
(defun cadadr (x) (car (cdr (car (cdr x))))) 
(defun caddar (x) (car (cdr (cdr (car x))))) 
(defun cadddr (x) (car (cdr (cdr (cdr x))))) 
(defun cdaaar (x) (cdr (car (car (car x))))) 
(defun cdaadr (x) (cdr (car (car (cdr x))))) 
(defun cdadar (x) (cdr (car (cdr (car x))))) 
(defun cdaddr (x) (cdr (car (cdr (cdr x))))) 
(defun cddaar (x) (cdr (cdr (car (car x))))) 
(defun cddadr (x) (cdr (cdr (car (cdr x))))) 
(defun cdddar (x) (cdr (cdr (cdr (car x))))) 
(defun cddddr (x) (cdr (cdr (cdr (cdr x))))) 

(defun first (list) (car list))
(defun second (list) (car (cdr list)))
(defun third (list) (car (cddr list)))
(defun fourth (list) (car (cdddr list)))
(defun fifth (list) (car (cddddr list)))
(defun sixth (list) (car (cdr (cddddr list))))
(defun seventh (list) (car (cddr (cddddr list))))
(defun eighth (list) (car (cdddr (cddddr list))))
(defun ninth (list) (car (cddddr (cddddr list))))
(defun tenth (list) (car (cdr (cddddr (cddddr list)))))
