# Pixelveil

A performant steganography and steganalysis library for hiding data in images, audio files, and videos!

Steganography can be used for watermarking, covert communications and info hiding in general. It is a relatively small but very versatile field in programming. This is a passion project I develop to learn, improve, and create steganography methods.

## The Currently Supported Steganography Methods

## Usage in Rust

Please consult the Rust crate documentation.

## Usage in Python

This library leverages [PyO3](https://github.com/PyO3/pyo3) to write python wrappers to the public interface of the Rust crate.

Even if you don't program in Rust, I encourage you to read the rust documentation of this library to have a good understanding of the Rust code that is called under the hood when the Python package is used.

Please note that `pyo3_runtime.PanicException` can be raised if the underlying Rust program panics, it should not be raised. If it does get raised, contact me.

All the Python functions have docstrings that you should read before using the function.

### Modules

Each module name matches the name of the module it wraps. For example, the `bpcs` module in the Rust crate has a wrapper module that is named `bpcs`.

In the Rust crate, there is a whole module tree that might be hard to navigate.Because of this, the PyO3 modules are all at the top level of the Python package. This was solved in Rust using re-exports.

### Functions

Each function is contained by the same module, and has the exact name as the Rust function it wraps. The argument types are slightly altered to adhere to Python's type system, but their order stays mostly the same.

## Contributing

As of writing this README, there is no contributing program for this repository. If you are interested in contributing, you contact me at `jebbex42@gmail.com`.

## Future Development

Because I am a One Man Operation™, I need time to research, prototype, and finalize each one of these steganography methods. So don't hold your breath :)
