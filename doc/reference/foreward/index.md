---
title: hrafn - a minimal LISP environment
---

### **Foreward**

<hr>

Welcome to *hrafn*. We hope it serves you well, and is easily extended to meet your needs.

Dynamic languages for low overhead environments must be lightweight, but developing applications in the target environment can be painful. *hrafn* is a general development and execution environment for containers, shells, and other utilities that need small installation and runtime footprints.

*hrafn* was designed to operate effectively on *Raspberry Pi* class machines and containers, and consists of a small native-code kernel runtime and a set of layered libraries in source form.

The *hrafn* runtime is a native code library that directly interprets *mu* kernel code. A great deal of useful work can be done directly with the limited kernel language, though it lacks the niceties of a more canonical LISP programming environment. More advanced language features require the use of the included *core* library. 

The *preface* library provides traditional LISP forms and a collection of functional programming features like pattern matching and monads. *hrafn* *core* and *preface* code are human-readable and portable between machine architectures.

Questions? You can contact the author at putnamjm.design@gmail.com.

<div style="page-break-after: always"></div>

### **Table of Contents**

------

**Foreward** .........................................................................................................</br>
**mu** ...................................................................................................................</br>
**core** ..................................................................................................................</br>
2.1 *about* ........................................................................................................... [2.1](2-1core.html)</br>
2.2 *reader* ......................................................................................................... [2.2](2-2reader.html)</br>
2.3 *compiler* ...................................................................................................... [2.3](2-3compile.html)</br>
2.4 *lambda* ........................................................................................................ [2.4](2-4lambda.html)</br>
2.5 *functions* ...................................................................................................... [2.5](2-5functions.html)</br>
2.6 *macros* ......................................................................................................... [2.6](2-6macros.html)</br>
2.7 *sequences* ..................................................................................................... [2.7](2-7sequences.html)</br>
2.8 *exceptions* .................................................................................................... [2.8](2-8exceptions.html)</br>
2.9 *lists* ............................................................................................................... [2.9](2-9lists.html)</br>
2.10 *streams* ....................................................................................................... [2.10](2-10streams.html)</br>
2.11 *utilities* ......................................................................................................... [2.11](2-11utilities.html)</br>
2.12 *core symbols* ............................................................................................... [2.12](2-12core-symbols.html)</br>
**preface** ............................................................................................................</br>
**hrafn** ...................................................................................................................</br>
**system** ..............................................................................................................</br>
**Appendices** ......................................................................................................</br>



