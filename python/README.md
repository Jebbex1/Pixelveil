# Pixelveil

A performant steganography and steganalysis library for hiding data in images, audio files, and videos!

Steganography can be used for watermarking, covert communications and info hiding in general. It is a relatively small but very versatile field in programming. This is a passion project I develop to learn, improve, and create steganography methods.

## The Currently Supported Steganography Methods

This library currently supports only [BPCS (Bit Plane Complexity Segmentation)](https://en.wikipedia.org/wiki/BPCS-steganography) steganography.

## Usage

This library leverages [PyO3](https://github.com/PyO3/pyo3) to write python wrappers for the public interface of the Rust crate.

Even if you don't program in Rust, I encourage you to read the Rust documentation of the underlying Rust crate to have a good understanding of the Rust code that is called under the hood when the Python package is used.

Please note that `pyo3_runtime.PanicException` can be raised if the underlying Rust program panics. If a program panics, it means that the arguments that were passed in are invalid. An explanation will be included in the error message. Every function that panics intentionally will have an explanation of the exact case in the docstring.

If you are certain that the arguments that you passed in are valid but the function still panicked, open an issue on he subject.

Also note that, all functions that require an image (or images) as an argument(s) expect to receive bytes that describe an image in a known file format (e.g. PNG, JPEG, BMP...). They also return bytes that describe an image in a known format.

All the Python functions have docstrings that I recommend you read before using the function.

### Modules

Each module name matches the name of the module it wraps. For example, the `bpcs` module in the Rust crate has a wrapper module that is named `bpcs`.

In the Rust crate, there is a whole module tree that might be hard to navigate.Because of this, the PyO3 modules are all at the top level of the Python package. This was solved in Rust using re-exports.

### Functions

Each function is contained by the same module, and has the exact name as the Rust function it wraps. The argument types are slightly altered to adhere to Python's type system, but their order stays mostly the same.

## Contributing

As of writing this README, there is no contributing program for this repository. If you are interested in contributing, contact me at `jebbex42@gmail.com`.

## Future Development

In the future I plan on implementing the following steganography methods (not in this particular order):

* Image LSB (Least Significant Bit) — only for lossless filetypes
* Lossless audio filetype (.wav) method(s)
* Lossy audio filetype (.mp3) method(s)
* Video steganography methods (.mp4)

Because I am a One Man Operation™, I need time to research, prototype, and finalize each one of these steganography methods. So don't hold your breath :)
