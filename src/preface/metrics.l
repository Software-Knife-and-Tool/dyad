;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  metrics: preface  metrics
;;;

;;;
;;; time macro
;;;
(defmacro time (form verbose)
  (let ((cform (core:compile form))
         (type-map
          '((:cons   . "conses     ")
            (:except . "exceptions ")
            (:func   . "functions  ")
            (:stream . "streams    ")
            (:symbol . "symbols    ")
            (:vector . "vectors    ")))
         (pre
          (list
           (list :cons (env:hp-type :heap :cons :in-heap) (env:hp-type :heap :cons :alloc))
           (list :except (env:hp-type :heap :except :in-heap) (env:hp-type :heap :except :alloc))
           (list :func (env:hp-type :heap :func :in-heap) (env:hp-type :heap :func :alloc))
           (list :stream (env:hp-type :heap :stream :in-heap) (env:hp-type :heap :stream :alloc))
           (list :symbol (env:hp-type :heap :symbol :in-heap) (env:hp-type :heap :symbol :alloc))
           (list :vector (env:hp-type :heap :vector :in-heap) (env:hp-type :heap :vector :alloc)))))
   
    (let ((start (system:utimes))
          (value (eval cform))
          (stop (system:utimes)))
      (if verbose
          (format t ";;; time: ~A consumes:~%time       : process ~Ams user ~Ams~%"
                  form
                  (fixnum- (svref stop 1) (svref start 1))
                  (fixnum- (svref stop 2) (svref start 2)))
          (format t "time        ~A ~A~%"
                  (fixnum- (svref stop 1) (svref start 1))
                  (fixnum- (svref stop 2) (svref start 2))))
      (mapc
       (lambda (type-info)
         (let* ((type (car type-info))
                (in-heap (nth 0 (cdr (assoc type pre))))
                (alloc (nth 1 (cdr (assoc type pre)))))
           (if verbose
               (if (not (eq in-heap (env:hp-type :heap type :in-heap)))
                   (format t "~A: ~A, ~A bytes~%"
                           (cdr type-info)
                           (fixnum- (env:hp-type :heap type :in-heap) in-heap)
                           (fixnum- (env:hp-type :heap type :alloc) alloc)))
               (format t "~A ~A ~A ~A ~A~%"
                       (cdr type-info)
                       in-heap
                       (env:hp-type :heap type :in-heap)
                       alloc
                       (env:hp-type :heap type :alloc)))))
       type-map)
      (list :quote value))))

;;;
;;; room
;;;
(defun room (verbosity)
  (if verbosity
      (progn
        (format t ";;; eko version: ~A~%" preface::eko-version)
        (format t ";;; automatic garbage collection is disabled~%")))
  (mapc
   (lambda (desc)
     (if verbosity
         (format t "~A: ~A in heap, (bytes ~A), free ~A~%"
                 (cdr desc)
                 (env:hp-type :heap (car desc) :in-heap)
                 (env:hp-type :heap (car desc) :alloc)
                 (env:hp-type :heap (car desc) :free))
         (format t "~A ~A ~A ~A~%"
                 (cdr desc)
                 (env:hp-type :heap (car desc) :in-heap)
                 (env:hp-type :heap (car desc) :alloc)
                 (env:hp-type :heap (car desc) :free))))
   '((:cons . "conses     ")
     (:except . "exceptions ")
     (:func . "functions  ")
     (:stream . "streams    ")
     (:symbol . "symbols    ")
     (:vector . "vectors    ")))
  verbosity)
