;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

(defun current-working-directory ()
  (system:cwd))

(defun chdir (path)
  (errorp-unless stringp path "chdir: not a string")
  (system::chdir path))

(defun exit (rc)
  (errorp-unless fixnump rc "exit: not a fixnum")
  (system:exit rc))

(defun logname ()
  (system:logname))

(defun process-environment ()
  (system:procenv))

(defun system (cmd)
  (errorp-unless stringp cmd "system:system not a string")
  (system:system cmd))

(defun utimes ()
    (system:utimes))
