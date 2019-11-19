// dirmod
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the License);
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an AS IS BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # dirmod
//! [![Travis-CI](https://travis-ci.com/SOF3/dirmod.svg?branch=master)](https://travis-ci.om/SOF3/dirmod)
//! [![crates.io](https://img.shields.io/crates/v/dirmod.svg)](https://crates.io/crates/dirmod)
//! [![crates.io](https://img.shields.io/crates/d/dirmod.svg)](https://crates.io/crates/dirmod)
//! [![docs.rs](https://docs.rs/dirmod/badge.svg)](https://sof3.github.io/dirmod/)
//! [![GitHub](https://img.shields.io/github/stars/SOF3/dirmod?style=social)](https://github.com/SOF3/dirmod)
//!
//! Tired of writing and updating all the `mod` statements in mod.rs?
//! Generate them with `dirmod` instead.
//!
//! `dirmod` scans your directory and generates the corresponding `mod` statements automatically
//! with a simple macro call:
//!
//! ```ignore
//! dirmod::all!();
//! ```
//!
//! And that's all!
//!
//! > *(Note: `dirmod` is designed for [Rust 2018 Edition][rust-2018],
//! so macros take simple and ambiguous names like `all`, `os`, etc.
//! It is recommended to call the macros in fully-qualified fashion
//! like `dirmod::all!()`, `dirmod::os!()`, etc. for clarity.
//! The old `#[macro_use] extern crate dirmod;` style is not recommended.)*
//!
//! ## Visibility
//! ### Default visibility
//! All modules can be set to a common visibility,
//! e.g. `pub mod` or `pub(self) mod`, etc. at your favour:
//!
//! ```ignore
//! dirmod::all!(default pub);
//! ```
//!
//! ### Re-exporting
//! You can also make all modules private, and set the visibility for the *re-exported* items instead:
//!
//! ```ignore
//! dirmod::all!(default pub use);
//! ```
//!
//! ### Separate file defaults and directory defaults
//! It might be common to handle file modules and directory modules separately:
//!
//! ```ignore
//! dirmod::all!(default file pub use; default dir pub);
//! ```
//!
//! This re-exports all items from file modules, and makes all directory modules public by name.
//! (This behaviour is similar to the package system in Go)
//!
//! ### The default behaviour
//! If the `default` argument is not given, `default file priv use; default dir priv` is the default
//! choice.
//!
//! ### Individual visibility
//! If there are individual modules among dozens that need special visibility configuration,
//! it is also possible to write:
//!
//! ```ignore
//! dirmod::all!(default pub; priv foo, bar);
//! ```
//!
//! Then all modules have `pub` visibility,
//! except `foo` and `bar` which are private.
//!
//! Similarly, if all modules are publicly re-exported and `foo` and `bar` are only exported as modules:
//! ```ignore
//! dirmod::all!(default pub use; pub foo, bar);
//! ```
//!
//! ## Conditional compilation
//! > But I use `mod` to implement conditional compilation!
//!
//! No problem, `dirmod` generates `cfg` attributes for some idiomatic styles:
//! - A directory where each module name is the feature name (e.g. `#[cfg(feature = "foo")] mod foo;`)
//! - A directory where each module name is the OS/OS family name (e.g. `#[cfg(target_family = "unix")] mod unix;`)
//!
//! This can be achieved by calling `dirmod::os!()`, `dirmod::family!()` or `dirmod::feature!()`.
//!
//! It is likely that different OS variants of the same module expose the same API,
//! so it might be practical to write:
//!
//! ```ignore
//! dirmod::os!(pub use);
//! ```
//!
//! If none of the modules support the current OS, you could trigger a compile error:
//!
//! ```ignore
//! dirmod::os!(pub use ||);
//! ```
//!
//! Or with a custom error message:
//!
//! ```ignore
//! dirmod::os!(pub use || "custom error message");
//! ```
//!
//! Note that it does not make sense to use the `||` on `dirmod::feature!`,
//! because Cargo features are incremental and should not be restricted in amount.
//!
//! [File an issue][gh-issues] if I missed any common styles!
//!
//! ## But I am still unhappy about xxxx corner case!
//! No problem, you don't have to use `dirmod` for every module.
//! `dirmod::all!()` has an `except` argument that excludes certain modules.
//! Since the macro simply generates `mod` statements,
//! it is perfectly fine to add more items before/after the macro call.
//!
//! ```ignore
//! dirmod::all!(except corge, grault);
//! ```
//!
//! ## Documentation
//! Instead of writing docs in mod.rs, write them in the module directly.
//! In addition to `dirmod` constraints, there are a few advantages:
//!
//! - Avoid lots of docs mixed together in a single mod.rs. Easier to navigate!
//! - Writing docs inside the module itself is much more relevant than references to the parent module.
//!
//! To write docs for the module, use this syntax at the top of the module (before any other items):
//!
//! ```ignore
//! //! Yay, I'm now describing myself!
//! //! I finally have my own place!
//! ```
//!
//! ## Supported Rust versions
//! Since detecting the source file requires the [`proc_macro_span`][proc-macro-span-issue] feature,
//! Rust Nightly is required to compile this crate.
//!
//! ## Examples
//! See the [`testcrate`][testcrate-blob] directory, which demonstrates the use of `dirmod::all!` and `dirmod::family!`.
//!
//! ## Syntax reference
//! A BNF syntax reference is available at [`syntax.bnf`][bnf-blob].
//!
//! ## Known unresolved issues
//! ### `rustfmt` support
//! `rustfmt` and `cargo fmt` operate on the modules directly included by the entry points
//! by detecting direct `mod` statements in the included files.
//! Since `rustfmt` does not expand (or even compile) macros ([known issue][rustfmt-issue]),
//! modules included by `dirmod` would not be formatted.
//!
//! The most straightforward alternative for now is to run `rustfmt src/**/*.rs`
//! with `shopt -s globstar` enabled on a Linux shell.
//!
//! ### Error reporting
//! The Rust compiler may fail to locate syntax error locations correctly
//! ([known issue][compiler-issue]).
//! However, this issue has only been reproduced with the specific case
//! where the syntax error is related to leading `#[]` which could be an inner attribute.
//!
//! [rust-2018]: https://doc.rust-lang.org/edition-guide/rust-2018/index.html
//! [gh-issues]: https://github.com/SOF3/dirmod
//! [proc-macro-span-issue]: https://github.com/rust-lang/rust/issues/54725
//! [testcrate-blob]: https://github.com/SOF3/dirmod/tree/master/testcrate
//! [bnf-blob]: https://github.com/SOF3/dirmod/blob/master/syntax.bnf
//! [rustfmt-issue]: https://github.com/rust-lang/rustfmt/issues/3253
//! [compiler-issue]: https://github.com/rust-lang/rust/issues/66071

pub use dirmod_codegen::*;
