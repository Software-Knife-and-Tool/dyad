---
title: print library
---

@comment_type ;;; %s
@title Appendix - System Format

@s System Format
<hr><br/>
<p></p>

--- system format --- noWeave
(defun system:format (stream fmt-string &rest args)
  (assert stringp fmt-string "system:format requires a string")
  (let*
     ((dest
        (cond
          ((null stream) (st-ostr ""))
          ((eq stream t) ())
          ((streamp stream) stream)
          (t (error stream "system:format requires a stream designator"))))
      (fmt (st-istr fmt-string))
      (radix-string (lambda (radix fix)
        (let ((out (st-ostr ""))
              (digits "0123456789abcdef"))
          ((lambda (radixf)
             (cond
               ((zerop fix) "0")
               (t (radixf radixf fix) (get-str out))))
           (lambda (fn fx)
             (cond
               ((zerop fx) ())
               (t (fn fn (car (trunc fx radix)))
                   (wr-byte (sv-ref digits (logand fx (fixnum- radix 1))) out)))))))))

     ((lambda (loop) (loop loop args))
      (lambda (loop args)
        (let ((ch (rd-byte fmt)))
          (cond
            ((null ch)
               (if stream
                   dest
                   (get-str dest)))
            ((eq ch #\~)
               (let ((dir (rd-byte fmt)))
                 (cond
                   ((null dir) (error dir "system:format eof while processing directive") :nil)
                   ((eq dir #\~) (wr-byte #\~ dest) (loop loop args))
                   ((eq dir #\%) (terpri dest) (loop loop args))
                   (t (cond
                        ((eq dir #\A) (print (mu:car args) dest ()))
                        ((eq dir #\C) (print (car args) dest ()))
                        ((eq dir #\S) (print (car args) dest t))
                        ((eq dir #\I) (system::pprint (car args) dest))
                        ((eq dir #\X)
                           (let ((fix (car args)))
                             (assert fixnump fix "system:format ~X requires a fixnum")
                             (print-str (radix-string 16 fix) dest ())))
                         ((eq dir #\D)
                            (let ((fix (car args)))
                              (assert fixnump fix "system:format ~D requires a fixnum")
                              (print-str (radix-string 10 fix) dest ())))
                         ((eq dir #\O)
                            (let ((fix (car args)))
                              (assert fixnump fix "system:format ~O requires a fixnum")
                              (print-str (radix-string 8 fix) dest ())))
                         ((eq dir #\W) (print (car args) dest :t))
                         ((eq dir #\a) (print-str (car args) dest ()))
                         ((eq dir #\s) (print-str (car args) dest t))
                         (t (error dir "unrecognized format directive")))
                        (loop loop (cdr args))))))
                (t (wr-byte ch dest) (loop loop args))))))))

(defun system::spaces (n stream)
  (assert fixnump n "system::spaces requires a fixnum")
  (assert streamp stream "system::spaces requires a stream")
  ((lambda (loop) (loop loop n))
   (lambda (loop n)
     (if (zerop n)
         ()
         ((lambda ()
            (wr-byte #\  stream)
            (loop loop (fixnum- n 1))))))))

(defun system::pprint (form stream)
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
           (list     1 1)))
        (stream (core::stream-designator stream)))
    ((lambda (loop) (loop loop form 0))
     (lambda (loop form indent)
       (system::spaces indent stream)
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
                       (t (core:warn form "system::pprint botch")))
                     (progn
                       (system:format stream "(")
                       (mapc (lambda (el) (loop loop el 1)) form)
                       (system:format stream ")")))))
             (t (system:format stream "~A~%" form))))
          (t (system:format stream "~A" form)))))
      ()))
---

--- system-format --- noWeave
@{system format}
---

--- system:format.l --- noWeave
@{system format}
---
