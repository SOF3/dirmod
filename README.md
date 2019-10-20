# dirmod
[![Travis-CI](https://travis-ci.com/SOF3/dirmod.svg?branch=master)](https://travis-ci.om/SOF3/dirmod)
[![crates.io](https://img.shields.io/crates/v/dirmod.svg)](https://crates.io/crates/dirmod)
[![crates.io](https://img.shields.io/crates/d/dirmod.svg)](https://crates.io/crates/dirmod)
[![docs.rs](https://docs.rs/dirmod/badge.svg)](https://sof3.github.io/dirmod/)

Tired of writing and updating all the `mod` statements in mod.rs?
Generate them with `dirmod` instead.

`dirmod` scans your directory and generates the corresponding `mod` statements automatically
with a simple macro call:

```rust
dirmod::all!();
```

*(Note: `dirmod` is designed for [Rust 2018 Edition](https://doc.rust-lang.org/edition-guide/rust-2018/index.html),
so macros takej simple and ambiguous names like `all`, `os`, etc.
It is recommended to call the macros in fully-qualified fashion
like `dirmod::all!`, `dirmod::os!()`, etc. for clarity.
The old `#[macro_use] extern crate dirmod;` style is not recommended.)*

## Visibility
Modules can be set to a common visibility,
so all modules can be `pub mod` or `pub(self) mod`, etc. by default at your favour:

```rust
dirmod::all!(pub);
```

You can also make all modules private, and set the visibility for the *re-exported* items instead.

If there are individual modules among dozens that need special visibility configuration,
it is also possible to write

## Conditional compilation
> But I use `mod` to implement conditional compilation!

No problem, `dirmod` generates `cfg` attributes for some idiomatic styles:
- A directory where each module name is the feature name (e.g. `#[cfg(feature = "foo")] mod foo;`)
- A directory where each module name is the OS/OS family name (e.g. `#[cfg(target_family = "unix")] mod unix;`)

[File an issue](https://github.com/SOF3/dirmod) if I missed any common styles!

## But I am still unhappy about Xxxx corner case!
No problem, you don't have to use `dirmod` for every module.
`dirmod::all!()` has an `except` argument that excludes certain modules.
Since the macro simply generates `mod` statements,
it is perfectly fine to add more items before/after the macro call.

## Documentation
Instead of writing docs in mod.rs, write them in the module directly.
In addition to `dirmod` constraints, there are a few advantages:

- Avoid lots of docs mixed together in a single mod.rs. Easier to navigate!
- Writing docs inside the module itself is much more relevant than references to the parent module.

To write docs for the module, use this syntax at the top of the module (before any other items):

```rust
//! Yay, I'm now describing myself!
//! I finally have my own place!
```

## Supported Rust versions
Since detecting the source file requires the [`proc_macro_span`](https://github.com/rust-lang/rust/issues/54725) feature,
Rust Nightly is required to compile this crate.

## How to use
See [the documentation](https://sof3.github.io/dirmod/) for detailed explanation.

## Examples
See the [`testcrate`](https://github.com/SOF3/dirmod/tree/master/testcrate) directory, which demonstrates the use of `dirmod::all!` and `dirmod::family!`.
