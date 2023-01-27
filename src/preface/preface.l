;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  preface: eko language
;;;
;;;  preface namespace
;;;
(in-namespace (ns "preface" (find-ns "core")))

(defun identity (obj) obj)

(defun funcall (fn &rest args)
  (core::funcall fn args))

(defun format (stream fmt-string &rest args)
  (core:format stream fmt-string args))

(defun append (&rest lists)
  (foldr
    (lambda (el acc)
      (if (listp el)
          (foldr (lambda (el acc) (cons el acc)) acc el)
          el))
      ()
      lists))

(defun apply (fn &rest args)
  (errorp-unless functionp fn "apply: not a function")
  ((lambda (loop) (fn (loop loop args)))
   (lambda (loop args)
     (if (cdr args)
         (cons (car args) (loop loop (cdr args)))
         ((lambda ()
           (errorp-unless listp (car args) "apply: not a list")
           (car args)))))))

(defun vector (&rest elements)
  (list-sv :t elements))

(defun list (&rest list) list)

;;; can we do this as a maplist?
(defun list* (&rest args)
  ((lambda (loop) (loop loop args))
   (lambda (loop args)
     (if (cdr args)
         (cons (car args) (loop loop (cdr args)))
         (car args)))))

(defmacro progn (&rest body)
  (list (list* 'lambda () body)))

(defmacro when (test &rest body)
  (preface:list 'if test (list* 'progn body)))

(defmacro unless (test &rest body)
  (preface:list 'if test () (list* 'progn body)))

(defmacro let (binds &rest body)
  (errorp-unless core:listp binds "let: not a binding list")
  (list*
    (list* 'lambda (mapcar car binds) body)
    (mapcar (lambda (el) (nth 1 el)) binds)))

(defmacro let* (binds &rest body)
  ((lambda (fn)
     (if binds
       (fn fn binds)
       (list* 'progn body)))
   (lambda (fn binds)
     (if (cdr binds)
         (list 'let (list (car binds)) (core::funcall fn (list fn (cdr binds))))
         (list* 'let (list (car binds)) body)))))

(defconst preface::gensym
  (let ((genstream (make-string-stream :output "0")))
    (lambda () ;;; this becomes a closure
      ((lambda (n)
         (core:format genstream "~A" (list (1+ n)))
         (symbol (core:format () "g~A" (list n))))
       (mu:read (make-string-stream :input (get-output-stream-string genstream)))))))

(defun gensym ()
  (core::funcall preface::gensym ()))

(defmacro bind-seq (pattern source &rest body)
  (errorp-unless consp pattern "bind-seq: pattern not a list")
  (let* ((source-gsym (gensym))
         (destruct
          (lambda ()
            (let ((nths
                   (foldr
                    (lambda (sym acc)
                      (errorp-unless symbolp sym "bind-seq: not a symbol")
                      (let* ((nth (car acc))
                             (phrases (cdr acc))
                             (phrase (list sym (preface:list 'elt source-gsym nth))))
                        (cons (1- nth) (cons phrase phrases))))
                    (cons (1- (length pattern)) ())
                    pattern)))
              (cdr nths)))))
    (list* 'let* (cons (preface:list source-gsym source) (destruct)) body)))

(defmacro cond (&rest clauses)
  (car
    (foldr
      (lambda (clause cond)
        ;;; (form) or (form . body)
        (errorp-unless consp clause "preface:cond not a cons")
        (let ((test (car clause))
              (body (cdr clause)))
          (if body
            (list (list* 'if test (list* 'progn body) cond))
            (list
              (let ((gsym (gensym)))
                (list 'let (list (list gsym test))
                  (list* 'if gsym gsym cond)))))))
    ()
    clauses)))

(defmacro and (&rest terms)
  (if terms
    (foldr
      (lambda (cns acc)
        (let ((term (car cns)))
          (if (cdr cns)
              (list 'if term acc)
              term)))
      ()
      (maplist identity terms))
    :t))

(defmacro or (&rest terms)
  (foldr
    (lambda (cns acc)
      (let ((term (car cns)))
        (if (cdr cns)
            (let ((gsym (gensym)))
              (list*
                (list
                  'let (list (list gsym (car cns)))
                  (list 'if gsym gsym acc))))
         term)))
    ()
    (maplist identity terms)))

;;;
;;; do macros
;;;
(defmacro do* (bind-form end-form &rest body)
  (errorp-unless consp end-form "do: binding form not a list")
  (mapc
   (lambda (phrase)
     (cond
       ((symbolp phrase)
        (errorp-unless uninternedp phrase "do: binding symbol is interned"))
       ((consp phrase)
        (errorp-unless (lambda (phrase) (fixnum< (length phrase) 4)) phrase "do: binding phrase format length")
        (errorp-unless uninternedp (nth 0 phrase) "do: binding phrase format type "))
       (t (error phrase "do: binding phrase type"))))
   bind-form)
  (errorp-unless consp end-form "do: end form not a list")
  (let ((self-gensym (gensym))
        (exit-gensym (gensym))
        (return-gensym (gensym))
        (var-list (mapcar (lambda (phrase) (if (symbolp phrase) phrase (nth 0 phrase))) bind-form))
        (init-list (mapcar (lambda (phrase) (if (symbolp phrase) () (nth 1 phrase))) bind-form))
        (step-list (mapcar (lambda (phrase) (if (symbolp phrase) () (nth 2 phrase))) bind-form)))
    (list 'fix*
          (list 'lambda (list* self-gensym exit-gensym return-gensym var-list)
                (list 'if (nth 0 end-form)
                      (list 'progn
                            (list 'env:fp-setv (list core::fn-frame-id self-gensym) 1 0)
                            (list 'env:fp-setv (list core::fn-frame-id self-gensym) 2 (nth 1 end-form)))
                      (list 'progn
                            (cons 'progn body)
                            (list 'env:fp-setv (list core::fn-frame-id self-gensym) 1 (list 'null exit-gensym))
                            (cons 'progn
                                  (maplist
                                   (lambda (step-cdr)
                                     (list 'env:fp-setv (list core::fn-frame-id self-gensym) (fixnum+ 2 (length step-cdr)) (car step-cdr)))
                                   step-list)))))
          (list 'list* () () (cons 'list init-list)))))

;;; (do let-form (end-test result) . body)

(defun preface::reclose (closure)
  (errorp-unless functionp closure "reclose: not a function")
  (let ((closed (core::lambda-closure (core::fn-lambda-desc closure))))
    (errorp-unless (lambda (list) (fixnum< 0 (length list))) closed "reclose environment botch")
    (env:fnv-set (car closed) 0 closure))
  closure)

(defmacro recur (name lambda &rest body)
  (let ((gsym (gensym)))
    (list 'let
          (list (list gsym
                      (list 'let
                            (list (list name (list 'lambda lambda)))
                            (list* 'lambda lambda body))))
          (list 'preface::reclose gsym))))

(defmacro letf (binds &rest body)
  (errorp-unless listp binds "letf: not a list")
  (list* 'let
         (mapcar
           (lambda (bind)
             (list (car bind)
                   (core::funcall recur bind)))
             binds)
         body))

(defmacro letf* (binds &rest body)
  (errorp-unless listp binds "letf*: not a list")
  ((lambda (fn)
     (if binds
       (fn fn binds)
       (list* 'progn body)))
   (lambda (fn binds)
     (if (cdr binds)
         (list 'letf (list (car binds)) (core::funcall fn (list fn (cdr binds))))
         (list* 'letf (list (car binds)) body)))))

(defmacro flet (binds &rest body)
  (errorp-unless listp binds "flet: not a list")
  (list* 'let
         (mapcar
          (lambda (bind)
            (list (nth 0 bind)
                  (list* 'lambda (nth 1 bind) (nthcdr 2 bind))))
          binds)
         body))

(defmacro flet* (binds &rest body)
  (errorp-unless listp binds "flet*: not a list")
  ((lambda (fn)
     (if binds
       (fn fn binds)
       (list* 'progn body)))
   (lambda (fn binds)
     (if (cdr binds)
         (list 'flet (list (car binds)) (core::funcall fn (list fn (cdr binds))))
         (list* 'flet (list (car binds)) body)))))

(defmacro defun-rec (name lambda &rest body)
  (errorp-unless symbolp name "defun: not a symbol")
  (errorp-when keywordp name "defun: not a symbol")
  (errorp-unless listp lambda "defun: not a lambda list")
  (format t "expands to: ~A~%" (list 'lambda lambda (list* 'letf (list (list* name lambda body)) body)))
  (core::def-symbol
        name
        (core:compile (list 'lambda lambda (list* 'letf (list (list* name lambda body)) body)))
        ())
  (list :quote name))

(defun compose (&rest fns)
  (lambda (arg)
    (foldl
     (lambda (fn acc)
       (funcall fn acc))
     arg
     fns)))

(defun complement (fn)
  (compose null fn))
