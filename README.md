# Pixelveil

A performant steganography and steganalysis library for hiding data in images, audio files, and videos!

Steganography can be used for watermarking, covert communications and info hiding in general. It is a relatively small but very versatile field in programming. This is a passion project I develop to learn, improve, and create steganography methods.

## The Currently Supported Steganography Methods

This library currently supports only [BPCS (Bit Plane Complexity Segmentation)](https://en.wikipedia.org/wiki/BPCS-steganography) steganography.

## Usage

This project deploys both a Rust crate and Python package.

### Rust Usage

Please consult the [Rust crate documentation](https://docs.rs/pixelveil/).

### Python Usage

Please consult the Python package [README.md](python/README.md).

## Contributing

As of writing this README, there is no contributing program for this repository. If you are interested in contributing, contact me at `jebbex42@gmail.com`.

## Future Development

In the future I plan on implementing the following steganography methods (not in this particular order):

* Image LSB (Least Significant Bit) — only for lossless filetypes
* Lossless audio filetype (.wav) method(s)
* Lossy audio filetype (.mp3) method(s)
* Video steganography methods (.mp4)

Because I am a One Man Operation™, I need time to research, prototype, and finalize each one of these steganography methods. So don't hold your breath :)
