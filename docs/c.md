# C interface

The relevant code is in the [c_api](../c_api) directory.

## Compiling the library

Run `make` (or do what the makefile does if not on Linux) to generate a static and
dynamic library as well as the corresponding header file. The files are located in
`output`.

If the provided functionality is not, enough check out the next section.

The [example_usage](../c_api/example_usage) directory contains a basic example using the
library from C++.

## Building your own library

The core Rust library [pauli_tracker] is highly generic; this feature is however lost
when creating a C API. Instead we have to implement more or less the same boilerplate
for every explicit type. This is where the two crates in [builder](../c_api/builder)
come in useful: [impl_api](../c_api/builder/impl_api/) can be used to automatically
implement all those boilerplate things automatically via macros.
[gen_bind] can then be used to automatically generate
the corresponding C header files (it can also generate C++, and Cython headers, but
they might need manual adjustments). Check out their (run `cargo doc --open` in the
correspinding directories) documentation and how we use them in
[pauli_tracker_lib](../c_api/pauli_tracker_lib) (the wrapper around [pauli_tracker]) and
[generate_bindings](../c_api/generate_bindings).

*Note that [gen_bind] is effectively just a simple, restricted wrapper around [cbindgen]
to help getting started. If things get a little bit more complicated one should use
[cbindgen] directly (it's really simple).*

### Notes about cbindgen

- Don't rename dependencies and don't name the wrapping library like a dependency. This
  makes problems is the resolution of types.

[pauli_tracker]: https://github.com/taeruh/pauli_tracker
[gen_bind]: ../bind/c_api/builder/gen_bind
[cbindgen]: https://github.com/mozilla/cbindgen
