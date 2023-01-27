---
title: 2.8 Core Exceptions
---

#### **Exceptions**

------

*core* exceptions are currently a thin layer on *mu* exceptions, and are currently not well-developed.



***:print-except*** *exception* *string* => *exception*

<hr>

<div class="list">
<span class="dfn">exception</span> : an <span class="dfn">exception</span> object</br>
<span class="dfn">string</span> : a <span class="dfn">string</span></br>
</div>



*print-except* prints a human-readable description of *exception* on *mu:std-out*, with *string* for additional identification.

*print-except* returns the *exception* argument.





***break*** *exception*

<hr>

<div class="list">
<span class="dfn">exception</span> : an <span class="dfn">exception</span> object</br>
</div>


*break* prints the *exception* via *:print-except* and enters an interactive loop.



##### Condition Reporting

<hr>
<div class="list">
<strong>error</strong> <span class="dfn">value string</span> <strong> => </strong><span class="dfn">value</span></br>
<strong>error-if</strong> <span class="dfn">test value string</span> <strong> => </strong><span class="dfn">value</span></br>
<strong>errorp-when</strong> <span class="dfn">predicate value string</span> <strong> => </strong><span class="dfn">value</span></br>
<strong>errorp-unless</strong> <span class="dfn">predicate value string</span> <strong> => </strong><span class="dfn">value</span></br>
<strong>warn</strong> <span class="dfn">value string</span> <strong> => </strong><span class="dfn">value</span></br>
<strong>debug</strong> <span class="dfn">value string</span> <strong> => </strong><span class="dfn">value</span></br>
</div>



These functions print error conditions. All of them except *warn* and *debug* call *break*.
