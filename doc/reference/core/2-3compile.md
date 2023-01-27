---
title: 2.3 Core Compiler
---

#### **Compiler**

------

The *core* compiler accepts all legal *mu* forms by the simple expedient of deferring any form it doesn't recognize to the runtime compiler. Consequently, *core* forms are a proper superset of *mu* forms. Anywhere a *core* form is legal, a *mu* form is recognized.

The *core* compiler adds enhanced special operators:

<div class="list">
lambda lists with rest arguments</br>
macro definition and expansion</br>
two and three clause <span class="dfn">if </span>form</br>
constant symbol binding form</br>
a primitive <span class="dfn">defun</span> special operator<br>
</div>



Compiling a *core* form results in an object that can be evaluated by the  *mu* evaluator.

##### *`[special operator]` * defconst *name* *form* => *symbol*

<hr>

<div class="list">
<span class="dfn">name</span> : a <span class="dfn">symbol</span>, unevaluated</br>
<span class="dfn">form</span> : a <span class="dfn">core form</span>, evaluated</br>
<span class="dfn">symbol</span>: <span class="dfn">name</span></br>
</div>



The *defconst* special operator binds *name* in the designated namespace to the value of *form* when evaluated. Multiple invocations of *defconst* on the same symbol have undefined consequences. Do not confuse *defconst* with the similarly named Common Lisp macro, it has very different semantics due to the lack of dynamic variables in *hrafn*.



##### *`[special operator]` * defun *name* *lambda-list* &rest *body* => *symbol*

<hr>

<div class="list">
<span class="dfn">name</span> : a <span class="dfn">symbol</span>, unevaluated</br>
<span class="dfn">lambda-list</span> : a [lambda list](2-4lambda.html), unevaluated</br>
<span class="dfn">body</span>: a list of <span class="dfn">forms</span>, unevaluated</br>
<span class="dfn">symbol</span> : a <span class="dfn">name</span></br>
</div>



The *defun* special operator binds *name* in the designated namespace to a function defined by the *lambda-list* and *body* arguments. Unlike the Common Lisp macro, the body is not surrounded in a block form, nor is the *name* symbol lexically visible in the body. Multiple invocations of *defun* on the same symbol have undefined consequences.



##### *`[special operator]`* defmacro *name* *lambda-list* &rest *body* => *symbol*

<hr>

<div class="list">
<span class="dfn">name</span> : a <span class="dfn">symbol</span>, unevaluated</br>
<span class="dfn">lambda-list</span> : a [lambda list](2-4lambda.html), unevaluated</br>
<span class="dfn">body</span>: a list of <span class="dfn">forms</span>, unevaluated</br>
<span class="dfn">symbol</span> : a <span class="dfn">name</span></br>
</div>



The *defmacro* special operator binds *name* in the designated namespace to a macro expander function defined by the *lambda-list* and *body* arguments. Multiple invocations of *defmacro* on the same symbol have undefined consequences.


##### *`[special operator]`*  if *test-form* *then-form* [*else-form*] => *value*

<hr>

<div class="list">
<span class="dfn">test-form</span> : a <span class="dfn">core</span> form, unevaluated</br>
<span class="dfn">then-form</span> : a <span class="dfn">core</span> form, unevaluated</br>
<span class="dfn">else-form</span> : a <span class="dfn">core</span> form, unevaluated</br>
</div>



The *if* special operator evaluates *test-form*. If the result is true as a generalized boolean and returns the evaluated *then-form*. In this case, *else-form* remains unevaluated. If the result of evaluating *test-form* is (), the evaluated *else-form* is returned.

*if* may be called without an *else-form*. If so and *test-form* evaluates to (), () is returned. This *if* form is similar to the Common Lisp special operator.


##### *`special operator`*  lambda  *lambda-list* &rest *body* => *function*

<hr>

<div class="list">
<span class="dfn">lambda-list</span> : a [lambda list](2-4lambda.html), unevaluated</br>
<span class="dfn">body</span>: a list of <span class="dfn">forms</span>, unevaluated</br>
</div>



The *lambda* special operator creates a function from *lambda-list* and *body* and returns it.


##### compile *form* => *value*

<hr>

<div class="list">
<span class="dfn">form</span> : a <span class="dfn">core form</span></br>
</div>


*compile*  compiles *form* in the null lexical environment and returns the compiled *form*.