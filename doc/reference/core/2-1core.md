---
title: 2. Core library

---

#### **About the Core Library**

------

*Core* is a syntactic and semantic extension library coded in  the *mu* language primarily for the benefit of the *preface* library. It adds a complement of functions purposely missing from *mu*, an enhanced exception facility, a compiler for lambdas with rest arguments and function closures, and macros. *Preface* is intended to be largely, if not wholly coded on *core*.

*Core* symbols reside in the *core* namespace, which inherits *mu*. A complete listing of *core* external symbols can be found in the [*core symbol list*](2-13core-symbols.html).

##### Type Designators

<hr>

Functions and special operators described in this reference specify type designators for arguments and returned values. Most of these map directly onto the *mu* type class, but *core* documentation cites a few synthetic classes that are not strictly *mu* types.

- ***value*** designates any *core* or *mu* type, used to type a value that has no more specialized class.
- ***form*** is an ***value*** or a special operator application.
- ***list*** is either a proper list or ***()***.
- ***byte*** is a small *fixnum* in the range of [0..255].
- ***boolean*** indicates one of ***t***  or ***:t*** for *true* and ***()*** or ***:nil*** for  *false*. 

- ***generalized boolean*** indicates ***()*** or ***:nil*** as *false*, and otherwise *true*.

- ***string*** is a *vector* of *characters*. *mu* has no separate string type.

- ***function-designator*** is a *mu* function type or a *symbol* bound to a function.

- ***stream-designator*** is a  *mu*  *stream* or a ***boolean*** which maps to a standard stream.
- ***sequence*** is either a ***list*** or a ***vector***.



##### Synopsis of *core* additions to the *mu* language

<hr>

<div class="list">
reader resolves unqualified non-lexical symbols to the current namespace</br>
folds and utility functions for sequences</br>
maps and utility functions for lists</br>
debug repl</br>
simple format facility</br>
stream designators</br>
improved exception handling, break loop</br>
lambdas with rest arguments</br>
macro definition and expansion</br>
file loader</br>
</div>
