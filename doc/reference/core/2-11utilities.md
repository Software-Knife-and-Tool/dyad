---
title: 2.11 Core Utilities

---

#### **Utilities**

<hr>
Functions united by the singular attribute of not obviously belonging anywhere else.


**load** *file-path verbose print*  ***=>*** *boolean*

<hr>

<div class="list">
<span class="dfn">file-path</span>: a <span class="dfn">string</span> file system path</br>
<span class="dfn">verbose</span>: a <span class="dfn">generalized boolean</span></br>
<span class="dfn">print</span>: a <span class="dfn"> generalized boolean</span></br>
</div>
*load* sequentially reads *core* forms from *file-path*, compiles them, and evaluates them. If *verbose* is *true*, a brief informational header is printed to *mu:std-out* before the load begins. If *print* is *true*, the result of evaluating each form is printed to *mu:std-out*. If *load* encounters an error, it prints an exception message and returns (). *load* catches exceptions and terminates after printing an informational message.

*load* returns *t* if it completes with no errors, otherwise *()*.





**format**  *dest* string arg-list*  ***=>*** *string | ()*

<hr>
<div class="list">
<span class="dfn">dest</span>: a <span class="dfn">boolean</span>, select output type</br>
<span class="dfn">string</span>: a format <span class="dfn">string</span></br>
<span class="dfn">arg-list</span>: a <span class="dfn">list</span> of arguments</br>
</div>

*format* determines whether it writes to a fresh *string* (if *dest* is ()) or to *mu:std-out* (if *dest* is *t*).

It then iterates the format *string* and formats successive elements of the *arg-list* onto the output according to the current directive. *format* prints non ***~*** characters as themselves. When *format* encounters a ***~,*** it uses the next character to determine how to print the current element of the *arg-list*.



<div class="list">
<strong>~~</strong> - print #\~ unescaped</br>
<strong>~%</strong> - print newline</br>
<strong>~A</strong> - print unescaped object like <span class="dfn">princ</span></br>
<strong>~S</strong> - print escaped object like <span class="dfn">prin1</span></br>
<strong>~D</strong> - print fixnum in decimal</br>
<strong>~X</strong> - print fixnum in hexadecimal</br>
</div>

***~*** followed by any other character raises an error.



**:debug-repl**  ***=>*** *()*

<hr>

*debug-repl* is a bare-bones REPL primarily intended for debugging *core*. 
