---
title: 2.6 Core Macros
---

#### **Macros**

<hr>

Macro definitions are compiled with *core* lambdas and macro calls are expanded at compile time.



**:macroexpand-1** *form* ***=>*** *form*</br>
***macroexpand*** *form* ***=>*** *form*

<hr>

<div class="list">
<span class="dfn">form</span> : a <span class="dfn">core form</span></br>
</div>


*macroexpand* and *macroexpand-1* expand a form against a macro definition.

*macroexpand* expands a form until it no longer is a macro call, *:macroexpand-1*
expands the form just once. *forms* that are not macro calls are returned unchanged.

(Common Lisp adds an optional environment argument here, think about why.)




***macro-function*** *function-designator* ***=>*** *function | ()*

<hr>
<div class="list">
<span class="dfn">function-designator</span> : a <span class="dfn">function-designator</span></br>
</div>


*macro-function* determines whether *function-designator* specifies a macro expander function, and if so, returns it. Otherwise, it returns ().
