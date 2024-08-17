# Programming resources

This section contains useful programming resources, mainly centered around
`Rust`, as it is the language of choice here.

## Rust Cross-compilation

This guide nicely explains how to setup cross-compilation for Rust targetting
the Raspberry Pi architecture.

Very useful to shorten compilation times, as compiling from the Raspberry Pi
directly is definitely not fast, especially with Rust.

[Cross-compiling Rust for the Raspberry Pi](https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/)

## Libraries & Crates of interest

- [gpiod](https://docs.rs/gpiod/latest/gpiod/) - A crate that interfaces with
  the Linux GPIO character devices.
