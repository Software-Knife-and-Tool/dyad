---
title: 5. Building and running *hrafn*
---

*mu* is the core execution environment for the *hrafn* system and consists of the `mu-runtime` binary, runtime libraries `libmu.so` and `libmu.a` .  `mu-runtime` is a small driver program that implements a library loader and REPL on top of the *libmu* runtime, see `src/mu-runtime`. For specific instructions on building for a particular architecture, see the top level README file.

###  Runtime Development
<hr>
<p></p>

The runtime component is distributed as both a static and shared library.  The `world` target builds the release version of the runtime and prepares a distribution image that may be installed in `/opt/hrafn` on your system with the `install` target. The build system requires `gcc/g++ version 12` or better compilers are installed, and can be reached in your path via `g++-12`.


>        make world
>
>        sudo make install

The `release`, `debug`, and `profile` targets compile and link the `mu-runtime` binary and create `libmu.so` and `libmu.a` libraries. `mu-runtime` can run standalone with only the *libmu* symbols available (used mostly for debugging)
or load one or more libraries through the command line interface.

The `commit` and `valgrind` targets run several lint utilities and create a `valgrind` profile. They require the `clang-tidy`, `cppcheck` and `valgrind` utilities.

###  Runtime Libraries

------

The *core* and *preface* symbols are defined in a self-contained source files, `./hrafn/dist/hrafn:core.l` and `./hrafn/dist/hrafn:preface.l`, which are generated by the build system. The stock distribution supplies the most recent `core.l` and `preface.l`, libraries need to be regenerated only if you change the source.

> make dist

###  Generating the Documentation
<hr>
The `doc` target compiles a series of markdown files to HTML documents to `./hrafn/doc/reference/html`. The most recent html files are supplied with the distribution.

To generate docmentation from source you will need the `pandoc` program installed and found in your path. 

>        make doc

###  Running the Test Suite
<hr>
<p></p>

The distribution supplies a suite of tests, which are used for validation during development.

To run the test suite:

>        make tests/summary tests/report

###  Running mu-runtime
The `mu-runtime` source is found in `./src/mu-runtime`. `mu-runtime` supplies a simple repl used mostly
for debugging.

From the *mu-runtime* help:

>            OVERVIEW: mu-runtime - posix platform mu repl
>            USAGE: mu-runtime [options] [src-file...]
>
>            OPTIONS:
>              -h                           print this message
>              -v                            print version string
>              -b                           enter break loop
>              -l SRCFILE             load SRCFILE in sequence
>              -e SEXPR               evaluate SEXPR and print result
>              -q SEXPR               evaluate SEXPR quietly
>
>              -E name:value...  environment configuration
>
>              src-file...                load source files

To run the `mu-runtime` repl with *libmu* symbols only:

>            mu-runtime [-b]

To run the `mu-runtime` repl with *libmu* and *core* symbols:

>            mu-runtime -l dist/hrafn:core.l -b
