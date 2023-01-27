---
title: 2.2 Core Reader
---

#### **Reader**

------

The core reader converts a stream of characters to values, and resolves unqualified symbols in the current namespace.

##### Readtable

------

A readable similar to the Common Lisp initial readable is included. While core reader syntax may be altered by judicious modification of this readable, it is used to read in the *preface* and subsequent libraries and significant changes to it will probably cause read failures while trying to bootstrap the system.

The current readtable is formatted as an association list and looks like:

```
   '((#\0 . constituent)  (#\1 . constituent)  (#\2 . constituent)  
     (#\3 . constituent)  (#\4 . constituent)  (#\5 . constituent)  
     (#\6 . constituent)  (#\7 . constituent)  (#\8 . constituent)  
     (#\9 . constituent)  (#\: . constituent)  (#\< . constituent)
     (#\> . constituent)  (#\= . constituent)  (#\? . constituent)  
     (#\! . constituent)  (#\@ . constituent)  (#xa . wspace)
     (#xd . wspace)       (#xc . wspace)       (#x20 . wspace)
     (#\; . tmacro)       (#\" . tmacro)       (#\# . macro)
     (#\' . tmacro)       (#\( . tmacro)       (#\) . tmacro)
     (#\` . tmacro)       (#\, . tmacro)       (#\\ . escape)       
     (#\| . mescape)      (#\a . constituent)  (#\b . constituent)
     (#\c . constituent)  (#\d . constituent)  (#\e . constituent)
     (#\f . constituent)  (#\g . constituent)  (#\h . constituent)
     (#\i . constituent)  (#\j . constituent)  (#\k . constituent)  
     (#\l . constituent)  (#\m . constituent)  (#\n . constituent)
     (#\o . constituent)  (#\p . constituent)  (#\q . constituent)
     (#\r . constituent)  (#\s . constituent)  (#\t . constituent)
     (#\v . constituent)  (#\w . constituent)  (#\x . constituent)  
     (#\y . constituent)  (#\z . constituent)  (#\[ . constituent) 
     (#\] . constituent)  (#\$ . constituent)  (#\* . constituent)
     (#\{ . constituent)  (#\} . constituent)  (#\+ . constituent)  
     (#\- . constituent)  (#\/ . constituent)  (#\~ . constituent)  
     (#\. . constituent)  (#\% . constituent)  (#\& . constituent)
     (#\^ . constituent)  (#\_ . constituent)  (#\a . constituent)
     (#\b . constituent)  (#\c . constituent)  (#\d . constituent)
     (#\e . constituent)  (#\f . constituent)  (#\g . constituent)
     (#\h . constituent)  (#\i . constituent)  (#\j . constituent)
     (#\k . constituent)  (#\l . constituent)  (#\m . constituent)
     (#\n . constituent)  (#\o . constituent)  (#\p . constituent)
     (#\q . constituent)  (#\r . constituent)  (#\s . constituent) 
     (#\t . constituent)  (#\u . constituent)  (#\v . constituent)
     (#\w . constituent)  (#\x . constituent)  (#\y . constituent)
     (#\z . constituent)))
```

***Stream Designators***

<hr>

A *stream-designator* is either a *stream* object, or the symbol *t*, or (). The *t* designator is an alias for *mu:std-in*, and () is an alias for *mu:std-out*. Functions that take stream designators map the designator to the appropriate stream.



##### ***read*** *stream-designator* *eof-error* *eof-value* => *value*

------


<div class="list">
<span class="dfn">stream-designator</span>: an input <span class="dfn">stream-designator</span></br>
<span class="dfn">eof-error</span>: a <span class="dfn">generalized boolean</span></br>
<span class="dfn">eof-value</span>: a <span class="dfn"> generalized boolean</span></br>
</div>

*read* is the *core reader* external interface. If *eof-error* is *true*, an exception will be raised if *read* is called on a *stream* that is at end of file. If *eof-error* is *false*, *eof-value* will be returned on end of file.

##### ***:read*** *stream* => *value*

------

<div class="list">
<span class="dfn">stream</span>: an input <span class="dfn">stream</span></br>
</div>
*:read* is the internal implementation of the core reader.