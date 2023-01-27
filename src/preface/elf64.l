;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; elf64 utilities
;;;
(defun elf64-write-bytes (stream &rest bytes)
  (mapc
   (lambda (bytes)
     (cond
       ((charp bytes) (write-byte (ch-ansi bytes) stream))
       ((fixnump bytes) (write-byte bytes stream))
       ((stringp bytes)
        (do* ((i 0 (1+ i)))
             ((fixnum< (1- (length bytes)) i) ())
          (write-byte (ch-ansi (schar bytes i)) stream)))
       (t (error bytes "elf64-write-bytes: untranslated type"))))
   bytes))

(defun elf64-read (type elf64)
  (let ((elf64-read (lambda (type stream) (funcall preface:elf64-read type stream))))
    (cond
      ((eq type :half)
       (fixnum+ (read-byte elf64 t ()) (ash (read-byte elf64 t ()) 8)))
      ((eq type :word)
       (fixnum+ (elf64-read :half elf64) (ash (elf64-read :half elf64) 16)))
      ((eq type :u64)
       (fixnum+ (elf64-read :word elf64) (ash (elf64-read :word elf64) 32)))
      ((eq type :tag)
       (mu::of-tag (elf64-read :u64 elf64)))
      (t (error size "elf64-read: not a legal type")))))

(defun elf64-write (type n stream)
  (let ((elf64-write (lambda (type n stream) (funcall preface:elf64-write type n stream))))
    (cond
      ((eq type :half)
       (elf64-write-bytes stream
                    (logand n #xff)
                    (logand (ash n -8) #xff)))
      ((eq type :word)
       (elf64-write :half n stream)
       (elf64-write :half (ash n -16) stream))
      ((eq type :u64)
       (elf64-write :half n stream)
       (elf64-write :half (ash n -32) stream))
      ((eq type :tag)
       (elf64-write :u64 (mu::tag-of n) stream))
      (t (error size "el64-write: not a legal type")))))

(defun elf64-write-header (out size)
  (errorp-unless streamp out "elf64-write-header: not a stream")
  ;;; header 64 bit little endian current version
  (elf64-write-bytes out #x7f "ELF" 2 1 1 0 0 0 0 0 0 0 0 0) ;;; 0 e_ident
  (elf64-write :half 0 out)  ;;; 16 e_type
  (elf64-write :half 0 out)  ;;; 18 e_machine
  (elf64-write :half 1 out)  ;;; 20 e_version
  (elf64-write :half 0 out)  ;;; 22 e_entry
  (elf64-write :u64 0 out)   ;;; 24 e_phoff
  (elf64-write :u64 56 out)  ;;; 32 e_shoff
  (elf64-write :word 0 out)  ;;; 40 e_flags
  (elf64-write :half 0 out)  ;;; 42 e_ehsize
  (elf64-write :half 0 out)  ;;; 44 e_phentsize
  (elf64-write :half 0 out)  ;;; 46 e_phnum
  (elf64-write :half 0 out)  ;;; 48 e_shentsize
  (elf64-write :half 0 out)  ;;; 50 e_shnum
  (elf64-write :half 0 out)  ;;; 52 e_shstrndx
  (elf64-write :word 0 out)  ;;; 56 sh_name
  (elf64-write :word 0 out)  ;;; 60 sh_type
  (elf64-write :u64 0 out)   ;;; 68 sh_flags
  (elf64-write :u64 0 out)   ;;; 76 sh_addr
  (elf64-write :u64 0 out)   ;;; 84 sh_offset
  (elf64-write :u64 size out)   ;;; 92 sh_size
  (elf64-write :word 0 out)  ;;; 100 sh_link
  (elf64-write :word 0 out)  ;;; 102 sh_info
  (elf64-write :u64 0 out)   ;;; 110 sh_addralign
  (elf64-write :u64 0 out))  ;;; 118 sh_entsize

(defun elf64-read-header (elf64)
  (mu:with-ex
      (lambda (ex)
        (format t "error: exception while reading header~%")
        (print-except ex "read-header")
        ())
    (lambda ()
      (let ((top
             (list
              (cons "magic" (elf64-read :u64 elf64))
              (cons "class" (read-byte elf64 t ()))
              (cons "data-encoding" (read-byte elf64 t ()))
              (cons "file-version" (read-byte elf64 t ()))
              (cons "OS-ABI" (read-byte elf64 t ()))
              (cons "ABI-version" (read-byte elf64 t ())))))
        (dotimes (i 6) (read-byte elf64 t ()))
        (append top
                (list
                 (cons "e_object" (elf64-read :half elf64))
                 (cons "e_machine" (elf64-read :half elf64))
                 (cons "e_version" (elf64-read :half elf64))
                 (cons "e_entry" (elf64-read :half elf64))
                 (cons "e_phoff" (elf64-read :u64 elf64))
                 (cons "e_shoff" (elf64-read :u64 elf64))
                 (cons "e_flags" (elf64-read :word elf64))
                 (cons "e_ehsize" (elf64-read :half elf64))
                 (cons "e_phentsize" (elf64-read :half elf64))
                 (cons "e_phnum" (elf64-read :half elf64))
                 (cons "e_shentsize" (elf64-read :half elf64))
                 (cons "e_shnum" (elf64-read :half elf64))
                 (cons "e_shstrndx" (elf64-read :half elf64))
                 (cons "sh_name" (elf64-read :word elf64))
                 (cons "sh_type" (elf64-read :word elf64))
                 (cons "sh_flags" (elf64-read :u64 elf64))
                 (cons "sh_addr" (elf64-read :u64 elf64))
                 (cons "sh_offset" (elf64-read :u64 elf64))
                 (cons "sh_size" (elf64-read :u64 elf64))
                 (cons "sh_link" (elf64-read :word elf64))
                 (cons "sh_info" (elf64-read :word elf64))
                 (cons "sh_addralign" (elf64-read :u64 elf64))
                 (cons "sh_entsize" (elf64-read :u64 elf64))))))))

(defun elf64-print-header (elf64-header)
  (errorp-unless consp elf64-header "print-header: not an association list")
  (mapc
   (lambda (entry)
     (format t "    ~A: ~A~%" (car entry) (cdr entry)))
   elf64-header))

(defun elf64-dump (path)
  (errorp-unless stringp path "elf64-dump: not a pathname")
  (with-open-file elf64 path :input
     (let ((magic (read-byte elf64 () ())))
       (if (or (null magic) (eofp elf64) (not (eq #x7f magic)))
           (format t "~A: not a elf64 file, magic #x~X~%" path magic)            
           (progn
             (format t "~A: elf64 file (magic #x~X ~A~A~A)~%" path magic
                     (ansi-ch (read-byte elf64 t ()))
                     (ansi-ch (read-byte elf64 t ()))
                     (ansi-ch (read-byte elf64 t ())))
             (format t "~A: elf64 file header~%" path)
             (elf64-print-header (elf64-read-header elf64))
             (format t "~A: body:~%" path)
             (do* ((tag (elf64-read :tag elf64) (elf64-read :tag elf64)))
                  ((eofp elf64) t)
               (format t "[~A]~%" (type-of tag) tag)))))))
