# *dyad* - a system programming environment


### Under heavy development

*dyad* is a LISP-idiomatic functionally-oriented interactive environment for programming in the Rust ecosystem. It is targeted to low-resource persistent POSIX environments.

*dyad* is a LISP-1 namespaced programming environment with Common Lisp idioms and macro system.

While *dyad* has much in common with Scheme, it is meant to be familiar to traditional LISP programmers.

*dyad* is a 2-Lisp, which gives it considerable flexibility in implementation and subsequent customization. A small, native code runtime kernel supports system classes, function application, heap/system image management, and the FFI framework.



#### Goals
------

- minimal POSIX runtime suitable for containers
- small and simple installation, no external dependencies
- add interactivity and extensibility to application implementations
- optional Rust FFI system
- resource overhead equivalent to a UNIX shell
- minimal external crate dependencies



#### State of the *dyad* system

------

*dyad* is a work in progress.

*dyad* should build with rust 1.68 or better. *dyad* builds are targeted to:

- x86-64 and AArch-64 Linux distributions
- x86-64 and M1 MacOs X
- x86-64 WSL
- Docker Alpine containers

Portability, libraries, deployment, documentation, and garbage collection are currently the top priorities.

See the documentation for an up to date list of what works and what doesn't.



#### About *dyad*

------

*dyad* is an immutable, namespaced *LISP-1* that borrows heavily from *Scheme*, but is stylistically more closely related to the Common LISP family of languages. *dyad* syntax and constructs will be familiar to the traditional LISP programmer. 

*dyad* leans heavily on functional programming principles.

The *dyad* runtime kernel is written in mostly-safe `rust` (the system image heap facility *mmaps* a file, which is an inherently unsafe operation.)

The *mu* runtime implements 64 bit tagged pointers and can accommodate an address space up to 59 bits. *libmu* is available as a crate, extends a Rust API for embedded applications, and is an evaluator for the kernel language *mu*. *mu* provides the usual fixed-width numeric types, lists, fixed-arity lambdas, exceptions, symbol namespaces, streams, and specialized vectors in a garbage collected environment.

The *dyad* 2-LISP system is organized as a stack of compilers. 

The *core* library provides *rest* lambdas, closures, defun/defconst/defmacro and a compiler for those forms.

*preface* extends *core* with various lexical binding forms, *cond/and/or/progn*, and a library loading facility.

Optional libraries provide a variety of enhancements and services, including Common LISP macros and binding special forms.



#### Viewing the documentation

------

The *dyad* documentation is a collection of *markdown* files in `doc/reference`, currently not complete. The *html* version of those files is available in the `doc/html` directories in both the source and release directories.

To browse the documentation, start with `dyad.html`.



#### Building and installing the *dyad* system

------

The *dyad* runtime *libmu* is a native code program that must be built for the target CPU architecture. The *dyad* build system requires only a `rust` compiler,`rust-fmt`,`clippy` and some form of the `make` utility. Other tools like  `valgrind` are optional.

Tests and performance measurement requires some version of `python 3`.

```
git clone https://github.com/Software-Knife-and-Tool/dyad.git
```

After cloning the *eko* repository, the *rust* system can be built and installed with the supplied makefile.

```
% make world
```

Having built the distribution, install it in `/opt/dyad`.

```
% sudo make install
```

Related build targets, `debug` and `profile`, compile for debugging and profiling respectively.

`make` with no arguments prints the available targets.

If you want to repackage *dyad* after a change to the library sources:

```
% make dist
```

and then install.



#### Testing and performance metrics

------

The distribution includes a test suite, which should be run after every interesting change and prior to any check in. The test suite consists of a couple of hundred or so individual tests separated into multiple sections, roughly separated by namespace.

Performance metrics are not yet implemented.

Failures in the early tests are almost guaranteed to cause complete failure of subsequent tests.

```
% make tests/summary
```

The output of the failing tests can be had with

```
% make tests/report
```

------

Performance metrics, which can take in excess of ten minutes to run, can be captured with

```
% make -C perf base
```

establishes a new baseline from the current metrics. Typically, you'd establish a baseline, make a change, and run the perf tests again to see how your changes affected the performance tests. Test results are not checked into the project. Once you've done that, subsequent perf runs will show the difference between the established baseline and the current run.

```
% make -C perf perf
```

Metrics include the average amount of time taken for an individual test and the number of objects allocated by that test. Differences between runs in the same installation can be in the 10% range. If you want an extended test, the NTESTS environment variable controls how many runs are included in a single test. The default NTESTS is 20.

```
% make -C perf summary
```

produces a synopsis of the difference between the current binaries and sources and the established baseline along with other interesting statistics.

For convenience, the *eko* Makefile provides

```
% make perf/base      # establish an NTESTS=50 baseline, will take some time to run
% make perf/perf      # run a shorter perf test, NTESTS=20
% make perf/report	  # compare baseline and most recent perf run
```



#### Running the *dyad* system

------

The *dyad* binaries, libraries, and source files are installed in `/opt/dyad` . The `bin` directory contains the binaries and shell scripts for running the system.

```
mu-runtime    runtime binary, minimal repl
dyad         shell script for running the extended repl
```


```
OVERVIEW: mu-runtime - posix platform mu interface
USAGE: mu-runtime [options] [src-file...]

OPTIONS:
  -h                   print this message
  -?                   print this message
  -v                   print version string and exit
  -p                   pipe mode, no welcome message or prompt
  -s                   script mode, do not enter break loop
  -l SRCFILE           load SRCFILE in sequence
  -e SEXPR             evaluate SEXPR and print result
  -q SEXPR             evaluate SEXPR quietly
  -c name:value[,...]  environment configuration  	   
  src-file ...         load source files
  
```

An interactive session for the extended *dyad* system is invoked by the`dyad` shell script, `:h` will print the currently available repl commands. Forms entered at the prompt are evaluated and the results printed. The prompt displays the current namespace.

```
% /opt/dyad/bin/dyad
;;; Dyad LISP version 0.0.2 (preface:repl) :h for help
user>
```

*rlwrap* makes the *eko* and *mu-runtime* repls much more useful, with command history and line editing.

```
% alias ,dyad='rlwrap -m /opt/dyad/bin/dyad $*'
```

Depending on your version *rlwrap*, may exhibit odd echoing behavior. Adding

```
set enable-bracketed-paste off
```



to your `~/.inputrc` may help.

------

*dyad* is named for the Norse ravens, which symbolize a wise, all-knowing messenger.

*LISP* is a path, one of many. *LISPs* are intentionally dynamic which has selected against them for use in production environments, yet statically-typed languages produce systems that are hard to interact with and even harder to change *in situ*.

Change and evolution are the only defenses a system has against obsolescence.

Dynamic systems are the next step.
