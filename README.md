# CFS App in Rust
This project is a prototype CFS application written in Rust.


The best case here is that this turns into a way to create new
CFS Apps in Rust, taking care of the boilerplate, the binding
generation, and integration with the CFS system.


## Building
This project has been tested with the i686-unknown-linux-gnu target
by building the POSIX version of OSAL.


To get this target using rustup, type
```shell
rustup target add i686-unknown-linux-gnu
```

This project also currently assumes you have the CFE repo as a submodule.


##The Concept
This library is build using a build.rs file which uses bindgen to generate
binding to all of the OSAL, PSP, and CFE includes you would need to write
a CFE application.


Since each CFS project may be different, the bindings should really be
generated each time as part of the build process. In principal, however,
we could build bindings to a generic CFS project. This may require
assuming an operating system, or having a separate set of bindings, one
for each operating system. Even then, any changes to the PSP or mission
specific configuration would require regenerating the bindings, so this
may not be a good idea.



The library is configured to build a shared library, which in principal can
be loaded by CFE as a CFS application. This is yet to be seen.

## Enhancements
It would be better to allow the user to point to a CFS project, and build
bindings from that. If the app is integrated into the CFS project, the
environmental variables should be able to cover that.


There might be a way to integrate with CFS so that the app is built with the
makefile system and gets packaged up just like a normal application.
In this case the user simply needs cargo installed, and the correct toolchain.
This may allow the correct OS bindings in OSAL and PSP when doing binding
generation to make this a more general solution.


This may end up as a template for writing CFS apps, where you essentially clone the
app, or there is a builder that outputs an app of a particular name, and you
include it in your project as usual.


## Notes
About 5MB by default, with or without lto=true.


Gets down to 2MB with strip.


Other CFS apps are 8KB to 20KB or so.

## Macros in CFS
There are macros with arguments (aka inline functions) used in certain CFE modules.
There are listed below:
* ccsds.h
* cfs\_es.h
* cfs\_evs.h
* cfs\_tbl\_filedef.h
* cfs\_sb.h
* common\_types.h
* cfe\_platform\_cfg.h
* cfe\_psp\_module.h
 
