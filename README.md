# dirmod
[![Travis-CI](https://travis-ci.com/SOF3/dirmod.svg?branch=master)](https://travis-ci.om/SOF3/dirmod)
[![crates.io](https://img.shields.io/crates/v/dirmod.svg)](https://crates.io/crates/dirmod)
[![crates.io](https://img.shields.io/crates/d/dirmod.svg)](https://crates.io/crates/dirmod)
[![docs.rs](https://docs.rs/dirmod/badge.svg)](https://docs.rs/dirmod)

`dirmod` provides several convenience macros most useful in lib.rs, main.rs and mod.rs
to automatically declare `mod` statements for all the files in the directory.

## Features
- Automatic \*.rs and \*/mod.rs detection
- Customize visibility for all/specific modules
- Exclude specific modules
- Optionally generate re-exports (`pub use`) statements (instead of directly exposing the module) for all/specific modules
- Conditional compilation for `features`/`target_os`/`target_family` based on filename as parameters

## Supported Rust versions
Since detecting the source file requires the [`proc_macro_span`](https://github.com/rust-lang/rust/issues/54725) feature,
Rust Nightly is required to compile this crate.

## How to use
Here are some intuitive examples:

```rust

```

See [the documentation](https://docs.rs/dirmod) for detailed explanation.

## Examples
See the [`testcrate`](testcrate) directory, which demonstrates the use of `dirmod::all!` and `dirmod::family!`.
