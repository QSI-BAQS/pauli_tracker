# Foreign function interface for the Pauli-Tracker

**This repository has been moved into https://github.com/taeruh/pauli_tracker (c_lib
directory) and will be further developed there. This repository here is not under
development anymore.**

___

**Initial phase; really, really unstable at the moment**

The goal of this repository is to provide foreign language wrappers around
[pauli_tracker].

*Currently the focus is on providing a bare bone C API/ABI, since this can be used from
basically any language.*

If things are more mature and stable there will be some real packages; currently you
just have to clone the repo and include it manually in an application.


### Supported languages

- [C](./docs/c.md)

For languages which are not listed, it is probably possible to use the C API, sometimes
with more, sometimes with less troubles. From C++ one can use it basically as from C; in
Julia it can be done with the `@ccall` macro; in Python, for example, with Cython
("Python objects <-> real types" might be troublesome); etc.

[pauli_tracker]: https://github.com/taeruh/pauli_tracker


## How to cite

When you use the Pauli Tracker for your research please cite it (currently just by
linking to this repo; a paper is in progress).
