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

#![feature(proc_macro_span)]

extern crate proc_macro;

fn context(ctx: &'static str) -> impl Fn(syn::Error) -> syn::Error {
    move |err| syn::Error::new(err.span(), format!("Error during {}: {}", ctx, err))
}

macro_rules! decl {
    ($name:ident: $(#[$docs:meta])*) => {
        #[proc_macro]
        $(#[$docs])*
        pub fn $name(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let ret = match imp::$name(ts.into()) {
                Ok(ts) => ts,
                Err(err) => err.to_compile_error(),
            }.into();
            ret
        }
    };
}

decl!(all:
      /// Include all possible modules in the directory
      ///
      /// # Parameters
      /// The following parameter statements can be joined by semicolons.
      /// - `default $vis [use]`: All modules have `$vis` visibility by default,
      ///   where `$vis` can be the standard visibilities like `pub`, `pub(crate)`,
      ///   etc. The special `priv` keyword can be used to indicate private
      ///   visibility. If the `use` keyword is added behind the visibility,
      ///   modules will remain private, and `$vis use module::*;` statements
      ///   would be appended.
      ///   If this statement is not given, `priv` is assumed for default.
      /// - `$vis [use] $name1, $name2, ...`: The specified modules have `$vis`
      ///   visibility, different from the default visibility.
      ///   The format of `$vis [use]` is identical to that in `default`.
      /// - `except $name1 $name2 ...`: The specified modules are excluded.
      ///
      /// For simplicity, there is no special syntax to add doc comments.
      /// To document modules, either use the `//!` inner documentation
      /// syntax within the module file, or use `except` to exclude
      /// declaration and declare them separately from the macro call.
      ///
      /// # Examples
      /// ```ignore
      /// all!();
      /// ```
      ///
      /// ```ignore
      /// all!(default pub(crate); pub foo);
      /// ```
      ///
      /// ```ignore
      /// all! {
      ///     default pub(super);
      ///     pub(crate) foo, bar;
      ///     pub qux, corge;
      ///     priv lorem;
      ///     except ipsum;
      /// }
      /// ```
      );

decl!(os:
      /// Includes modules based on the `target_os` cfg attribute.
      ///
      /// Each module named `$mod` is conditionally compiled with the
      /// `#[cfg(target_os = $mod)]` option, allowing OS-specific module
      /// files/directories like `windows.rs`, `unix.rs`, etc.
      ///
      /// Note that this macro does not check for nonexistent `target_os`
      /// values, so incorrect usage will not lead to any warnings
      /// (and likely never compile the incorrect modules).
      /// See [this page](https://doc.rust-lang.org/reference/conditional-compilation.html)
      /// for a list of possible values.
      ///
      /// It is usually a good idea to provide the `use` keyword and expose
      /// the same API methods in all specific operating systems, preventing
      /// the need of `target_os` checking outside the crate.
      ///
      /// # Parameters
      /// ```ignore
      /// os!($vis [use] [|| [$error]]);
      /// ```
      ///
      /// `os!` accepts a visibility and an optional `use` keyword, with the
      /// same meaning as those in [`all!`](macro.all.html).
      ///
      /// The optional `|| $error` clause adds the code to check if at least
      /// one of the modules is compiled; otherwise,
      /// [`compile_error!`](https://doc.rust-lang.org/std/macro.compile_error.html)
      /// would be triggered. `$error` should be a string literal. If `$error`
      /// is omitted, it is replaced by the default message
      /// `"target_os must be one of \"xxx\", \"yyy\", ..."`,
      /// where xxx and yyy are the available modules.
      ///
      /// `os!` does not provide any filtering, and is intended for parent
      /// modules with only platform-specific submodules.  If non-OS-specific
      /// modules are desired, consider moving the OS-specific modules to the
      /// same directory.
      ///
      /// # Examples
      /// ```ignore
      /// os!(priv ||);
      /// ```
      ///
      /// ```ignore
      /// os!(pub use || "Unsupported operating system");
      /// ```
      ///
      /// If none of the modules in the directory get compiled, compilation
      /// would abort with the message "Unsupported operating system".
      );

decl!(family:
      /// Includes modules based on the `target_family` cfg attribute.
      ///
      /// This macro is identical to [`os!`](macro.os.html), except `target_os`
      /// is replaced with `target_family`, hence only accepting
      /// `windows.rs` and `unix.rs`.
      ///
      /// However, similar to `os!`, this macro does not validate values.
      /// Use the `||` syntax to report errors correctly.
      );

decl!(feature:
      /// Includes modules based on the `feature` cfg attribute.
      ///
      /// This macro has exactly the same semantics and format as
      /// [`os!`](macro.os.html) except it uses `#[cfg(feature = $module)]`
      /// instead of `target_os`.
      ///
      /// # Parameters
      /// ```ignore
      /// feature!($vis [use] [|| [$error]]);
      /// ```
      ///
      /// See [`os!](macro.os.html) for explanation of the parameter values.
      );

mod imp;
mod parse;
