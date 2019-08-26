# Crate simple-locale

[![travis.ci](https://travis-ci.org/johnstonskj/locale-settings.svg?branch=master)](https://travis-ci.org/johnstonskj/locale-settings)
[![crates.io](https://img.shields.io/crates/v/locale-settings.svg)](https://crates.io/crates/locale-settings)
[![docs.rs](https://docs.rs/locale-settings/badge.svg)](https://docs.rs/locale-settings)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
![mit License](https://img.shields.io/badge/license-mit-118811.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/locale-settings.svg)](https://github.com/johnstonskj/locale-settings/stargazers)

This crate provides a higher-level interface to locale settings, usually accessed via POSIX 
(see [ISO/IEC 15897](https://www.iso.org/standard/50707.html)) operating system functions. 

## Operating System Coverage

Currently only tested on macOS, Linux and Windows to come.

## Pre-Build Process

The following describe two code generation steps that are executed outside
the normal build process as the output is stored in Git and versioned 
based on external factors.

### FFI Bindings

As mentioned above, this crate depends on FFI bindings to POSIX locale
functions, and there are O/S differences that make this a pain. The script
[`create-bindings.sh`](https://github.com/johnstonskj/simple-locale/blob/master/create-bindings.sh)
is used to generate these bindings (using cargo bindgen) in such a way that
different O/S bindings can be built effectively.

## History

* **0.3.0** - reorganized module hierarchy.
* **0.1.0** - extracted from [simple-locale](https://github.com/johnstonskj/simple-locale).

## TODO

* Determine naming convention between the names in the `codes` and `settings`
  modules. 
  * Expect that the names in code modules will be changed to reflect
    those in the settings.
* Build and test for Linux.
  * How to deal with extended `LC_` categories (address, identification,
    measurement, name, paper, telephone). 
* Build and test for Windows.
