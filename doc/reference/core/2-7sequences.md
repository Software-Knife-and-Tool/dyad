---
title: 2.7 Core Sequences
---

#### **Sequences**

------

A *core* *sequence* is either a *list* or a *vector*.  *core* sequence functions operate identically on lists and vectors.



**length** *sequence* ***=>*** *fixnum*

<hr>
<div class="list">
<span class="dfn">sequence</span> : a <span class="dfn">core sequence</span></br>
</div>


*length* returns the number of elements in the sequence.



**foldl**   *function init-value sequence* ***=>** sequence*</br>
**foldr**  *function init-value sequence* ***=>*** *sequence*

<hr>
<div class="list">
<span class="dfn">function</span>:  a  <span class="dfn">function of 2 arguments</span></br>
<span class="dfn">init-value</span>:  a  <span class="dfn">core form</span></br>
<span class="dfn">sequence</span>:  a <span class="dfn">core sequence</span></br>
</div>



*core* folds iterate a sequence from either the left or right of the argument sequence. The *function* to the fold takes two arguments, the first is an element from the argument sequence and the second is an accumulated value. The fold function combines the element and the accumulated value and returns a new accumulated value. The first accumulated value is taken from the *init-value* argument.

*core* folds return the final accumulated value.



**findl-if** *function sequence*  ***=>*** *value*</br>
**findr-if**  *function sequence* ***=>*** *value*

<hr>
<div class="list">
<span class="dfn">function</span>:  a  <span class="dfn">function</span> of one argument</br>
<span class="dfn">sequence</span>:  a <span class="dfn">core sequence</span></br>
</div>




*core* finds iterate a sequence from either the left or the right of the sequence. The *function* to the find takes a single argument, the sequence element. If the find *function* returns true, that element is returned. If no element in the sequence meets the *function* criteria, () is returned.



**positionl**  *function item sequence*  ***=>*** *fixnum* | ()</br>
**positionr**  *function item sequence* ***=>*** *fixnum* | ()

<hr>
<div class="list">
<span class="dfn">function</span>:  a <span class="dfn">function</span> of two arguments</br>
<span class="dfn">item</span>:  an <span class="dfn">value</span></br>
<span class="dfn">sequence</span>:  a <span class="dfn">core sequence</span></br>
</div>



*core* position functions iterate a sequence from either the left or right. Elements of the sequence are compared to *item* with *function*, and if true the position is returned. If *item* is not found in the sequence, () is returned.
