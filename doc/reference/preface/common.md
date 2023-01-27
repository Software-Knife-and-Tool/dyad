---
title: common library
---

(defmacro common:dotimes (ctr-list &rest body)
  (core:assert core:consp ctr-list "common:dotimes requires an init list")
  (ix:let ((init (mu:nth 1 ctr-list))
           (ctr (mu:car ctr-list))
           (loop-gsym (ix:gensym)))
    (core:assert core:symbolp (mu:car ctr-list) "common:dotimes requires a symbol")  
    (ix:list
      (ix:list 'lambda (ix:list loop-gsym) (ix:list loop-gsym loop-gsym (ix:list 'core:eval init)))
      (ix:list 'lambda
                 (ix:list loop-gsym ctr)
                 (ix:list 'if
                            (ix:list core:zerop ctr)
                            ()
                            (ix:list 'ix:progn
                                       (ix:list* 'ix:progn body)
                                       (ix:list loop-gsym loop-gsym
                                                  (ix:list 'mu:fixnum- ctr 1))))))))

(defmacro common:dolist (init-list &rest body)
  (core:assert core:consp init-list "common:dolist requires an init list")
  (ix:let ((init (mu:nth 1 init-list))
             (sym (mu:car init-list)))
    (core:assert core:symbolp sym "common:dolist requires a symbol")  
    (core:assert core:listp init "common:dolist requires a list")
    (ix:list
      (ix:list 'lambda ()
        (ix:list
         'core:mapc
         (ix:list* 'lambda (ix:list sym) body)
         (ix:list 'core:eval init))
        ()))))

