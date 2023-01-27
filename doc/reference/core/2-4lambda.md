---
title: 2.4 Core Lambda
---

#### **Lambda**

------

*lambda* is the part of the *core* compiler that deals with function definition, in specific the lambda rest argument.



##### *core lambda form*

------

The *core lambda* form adds a rest argument to the *mu* fixed argument lambda syntax.

##### (lambda ([symbol ...] [&rest rest-symbol]) . body)





##### *lambda-descriptor*

------

A *lambda-descriptor* is a general vector that holds information about a lambda definition. A lambda descriptor is compiled into the body of a rest lambda or closure.

###### #(:t *lambda-syms req-syms rest-sym macrop environment closure*)

<div class="list">
<span class="dfn">lambda-syms</span> : a list of <span class="dfn">symbols</span> suitable for :lambda</br>
<span class="dfn">req-syms</span> : a <span class="dfn">list</span> of required <span class="dfn">symbols</span></br>
<span class="dfn">rest-sym</span>: rest <span class="dfn">symbol</span> or ()</br>
<span class="dfn">macrop</span> : a <span class="dfn">boolean</span></br>
<span class="dfn">environment</span> : a <span class="dfn">list</span> of lambda descriptors</br>
<span class="dfn">closure</span> : a <span class="dfn">list</span> of frames</br>
</div>










