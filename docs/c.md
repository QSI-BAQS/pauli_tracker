# C interface

The relevant code is in the [c_api](../c_api) directory.

## Compiling the library

Run `make` (or do what the makefile does if not on Linux) to generate a static and a
dynamic library as well as the corresponding header file. The files are located in
`output`.

Read the top level documentation of [pauli_tracker_clib]
(create it with `cargo doc --open` in the directory)!

**Note**: The compilation of the libraries requires a fairly new Rust version (>= 1.71)
and the generation of the header file requires that a nightly toolchain is installed.

The provided API does not represent the full API of [pauli_tracker], it's only a small
subset of it. Some things that should probably be in it, may be randomly missing. Feel
free to open an issue, or better a PR, if you think something should provided. The same
is applies to the [impl_api] helper crate (see below).

If the provided functionality is not enough, or if the used naming convention for
explicit generics is too annoying, check out the next section.

The [example_usage](../c_api/example_usage) directory contains a basic example using the
library from C++.

## Building your own library

The core Rust library [pauli_tracker] is highly generic; this feature is however lost
when creating a C API. Instead we have to implement more or less the same boilerplate
for every explicit type. This is where the two crates in [builder](../c_api/builder)
come in useful: [impl_api] can be used to automatically
implement many of those boilerplate things automatically via macros.
The [gen_bind] crate (a wrapper around [cbindgen]) can then be used to automatically
generate the corresponding C header files (it can also generate C++, and Cython headers,
but they might need manual adjustments). Check out their documentation (run `cargo doc
--open` in the correspinding directories) and how we use them in
[pauli_tracker_clib] (the wrapper around [pauli_tracker]) and
[generate_bindings](../c_api/generate_bindings).

You can use this to build your own library similar to how we do it. You don't have to
copy what is already provided; just build an additional library and link to both. In
this case the comment `/// cbindgen:ignore` might be handy, to prevent duplicate type
definitions.

Same as above, feel free to open issues, or better PRs, regarding additional
functionality for the [imp_api] crate.

*Note that [gen_bind] is effectively just a simple, restricted wrapper around [cbindgen]
to help getting started. If things get a little bit more complicated one should use
cbindgen directly (it's really simple) and gen_bind's cbindgen configuration maybe as
initial template. Also note, that while cbindgen does a pretty good job for most
use cases, one might to customize the output in certain cases.*

### Notes about cbindgen

- Don't rename dependencies and don't name the wrapping library like a dependency. This
  makes problems is the resolution of types.
- Default generic parameters are problematic, just specify them.

[pauli_tracker]: https://github.com/taeruh/pauli_tracker
[pauli_tracker_clib]: ../c_api/pauli_tracker_clib/
[gen_bind]: ../bind/c_api/builder/gen_bind
[cbindgen]: https://github.com/mozilla/cbindgen
[imp_api]: ../c_api/builder/impl_api/
