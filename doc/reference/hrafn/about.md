---
title: The *eko* Programming Environment
---

### Foreward

------

The *eko* programming environment is a collection of facilities for meta-programming.

Modern programming languages do a wide variety of things well, but most fail to support layered design
and domain-specific languages. Large programs are comprised as much as ten percent of their code as
meta-programming, yet many language systems don't have much internal support for it.

Meta-programming facilities in wide use today are the C++ and Haskell templating systems, various
annotation systems like Java's, and Python's decorators, usually supplemented by a suite of external tools.

LISP-derived languages like *Clojure* and *Scheme* have macro systems that can do arbitrary source
level transformations at compile time. Those macro systems have the advantage of using the host language
to program the transformation pipeline.

While many langauges incorporate functional features (higher order functions, monads, composition, etc)
most of them rely on programs based on mutable objects supplemented with an object system. There is a great
deal to gain by being mostly immutable and replacing objects with an effective type system and algebraic
data types.

*eko* explores functional meta-programming techniques by creating a stratified implementation of a programming
environment from the ground up. *eko* itself is constructed as a stack of language compilers, culminating in a
native code compiler. The majority of the system code is easily visible and can be as easily modified.

### About *eko*

------

*eko* is a program development platform for x86_64 and AArch64 POSIX operating systems. The *mu* runtime kernel is
written in C++20, compiles with recent g++ or clang++, requires only the standard library, and supports traditional
Lisp-style data types in a garbage collected environment.

The *mu* runtime implements 64 bit tagged pointers and can accommodate an address space up to 62 bits. *mu* is available
as a library, extends a C/C++ API for application embedding, and is source-level evaluator for the *mu* language, an
immutable namespaced Lisp-1. *mu* provides the usual fixed-width numeric types, lists, fixed-arity lambdas, exceptions,
streams, and specialized vectors.

The *core* language is written in *mu*, and provides variadic lambdas, closures, and macros.

The *preface* language, predictably written in *core*, provides lexical binding constructs, conditionals, structs,
and foreign function modules.

The *eko* language stack extends syntax and functions familiar to the traditional Lisp programmer while retaining the
conceptual simplicity of a Scheme. As such, *eko* is intended as a workbench for creating extensible programs.

<a href="toc.html">Table of Contents</a>
