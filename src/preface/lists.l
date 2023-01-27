;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; preface lists
;;;
(defun zip (list-1 list-2)
  (errorp-unless listp list-1 "zip: not a list")
  (errorp-unless listp list-2 "zip: not a list")
  (letf ((loop (l1 l2)
          (when l1
            (core::append
             (list (list (car l1) (car l2)))
             (loop (cdr l1) (cdr l2))))))
    (loop list-1 list-2)))

(defun copy-list (list)
  (errorp-unless listp list "copy-list not a list")
  (letf ((loop (list)
      (when list
        (cons (car list) (loop (cdr list))))))
        (loop list)))

(defun reverse (list)
  (errorp-unless listp list "reverse: not a list")
  (letf ((loop (list)
      (when list
        (core::append (loop (cdr list)) (cons (car list) ())))))
    (loop list)))
  
(defun filter (predicate list)
  (errorp-unless functionp predicate "filter: not a function")
  (errorp-unless listp list "filter: not a list")
  (letf ((loop (list)
      (when list
         (if (predicate (car list))
             (cons (car list) (loop (cdr list)))
             (loop (cdr list)))))))
    (loop list))

