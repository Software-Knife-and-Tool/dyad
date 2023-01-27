---
title: 2.9 Core Lists
---

#### **Lists**

<hr>
*Core* list functions are inspired by Common Lisp and Scala.


***append*** *list* ***=>*** *list*

<hr>

<div class="list">
<span class="dfn">list</span> : a <span class="dfn">list</span></br>
</div>



*append* merges the elements of *list* into a single list. All elements but the last must be lists.



***mapc*** *function* *list*=> *list*</br>
***mapcar*** *function*  src-list ***=>*** *list*</br>
***mapl***  *function* *src-list* ***=>*** *list*</br>
***maplist*** *function* *src-list* ***=>*** *list*

<hr>

<div class="list">
<span class="dfn">function</span> : a <span class="dfn">function</span> taking a single argument</br>
<span class="dfn">list</span> : a <span class="dfn">list</span></br>
</div>



*mapc* iterates a list and calls *function* with each element. Returns the *list* argument, side-effects only.

*mapcar* iterates a list and calls *function* with each element. The results are used to construct a new list.

*mapl* is similar to *mapc* except that *function* is called with the *cons* of each list element. Returns the *list* argument, side-effects only.

*maplist* is similar to *mapl* except that the results of calling *function* with the *src-list* conses are used to construct a new list which is returned. 





***sv-list*** *vector* ***=>*** *list*

<hr>



<div class="list">
<span class="dfn">vector</span> : a <span class="dfn">vector</span></br>
</div>


*sv-list* coerces (which is probably a better name for it, and moves it to *sequences*) a *vector* to a *list*.



***dropl*** *list* *n* ***=>*** *list*</br>
***dropr*** *list* *n* ***=>*** *list*

<hr>

<div class="list">
<span class="dfn">list</span> : a <span class="dfn">list</span></br>
<span class="dfn">n</span> : a <span class="dfn">fixnum</span></br>
</div>


*dropl* returns *list* without the first *n* elements, essentially the *nthcdr*.
*dropr* creates a new list from *list* without the last *n* elements. 



***assoc*** *item*  *a-list* ***=>*** *list*

<hr>

<div class="list">
<span class="dfn">item</span> : an <span class="dfn">value</span></br>
<span class="dfn">a-list</span> : an <span class="dfn">association list</span></br>
</div>



An *association list* is a list of dotted pairs, the car of which is the key value and the cdr the associated value.

*assoc* searches an association list for *item* using *eq* for the key value of each dotted pair. The first such successful comparison returns the corresponding associated value. If there are no matches, *assoc* returns ().



***last*** *list* ***=>*** *list*

<hr>
<div class="list">
<span class="dfn">list</span> : a <span class="dfn">list</span></br>
</div>



*last* returns the last *cdr* of *list*.  The return value is not copied.
