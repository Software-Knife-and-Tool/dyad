---
title: 2.5 Core Functions
---

#### **Functions**

<hr>

The *function* module is an adjunct to the core compiler. It is primarily responsible for compiling and executing function applications.

Rest argument or closure functions will be compiled with it's first form as the [lambda-descriptor](2-4lambda.html) from the defining lambda. Applications of those functions will subsequently be compiled as a call to the fixed-argument *:funcall* function, which arranges to manage the function's environment, evaluate and rewrite the arguments in a form that *mu:eval* can execute.

The *core* compiler compiles all forms so that they can be directly evaluated by the runtime evaluator. Lambdas with rest arguments, macro expander calls, and closures need to be treated differently. A specialized funcall is compiled into function activation forms that need it. Function calls with rest arguments are massaged into a fixed-arity function call that can be executed by the runtime.

##### :funcall *function* *arg-list* => *value*

------

<div class="list">
<span class="dfn">function</span> : a <span class="dfn">core function</span></br>
<span class="dfn">arg-list</span>: a list of <span class="dfn">core forms</span>, possibly ()</br>
</div>



*:funcall* manages the runtime's lexical environment for closures and creates appropriate argument lists for the runtime evaluator.
