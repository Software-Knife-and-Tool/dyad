;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; a state-like monad
;;;
;;; take a type and return a closure.
;;;
(defun state-frame (closure)
  (errorp-unless symbolp closure "preface::state-frame not a function")
  (let* ((desc (core::fn-lambda-desc closure))
         (frame (car (core::lambda-closure desc))))
    (errorp-unless (lambda (frame) (eq (type-of frame :frame))) "preface:state-frame botch")
    frame))

;;;
;;; (defstate :type value) =>
;;;
;;; (let ((type-gsym type)
;;;       (value-gsym (eval value)))
;;;   (lambda (cmd &rest arg)
;;;     (core:errorp-unless keywordp cmd "unknown cmd")
;;;     (cond cmd
;;;       ((eq cmd :get) value-gsym)
;;;       ((eq cmd :set) ....)
;;;       ((eq cmd :typeof) type-gsym)
;;;       (:t (core:error cmd "unknown cmd"))))
;;;
#|
(defmacro preface::state (type value)
  (errorp-unless symbolp type "preface::state not a type")
  (flet ((closure (fn &rest args)
                  (let ((state value))
                    (cond
                     ((eq fn :get) state)
                     ((eq fn :set) (env:set-env (preface::state-frame closure) 0 (car args)))
                     (t (error fn "preface::state unknown operator"))))))
        ;;; (errorp-unless (complement fn-unclosedp) closure "is not a closure")
        closure))
|#
