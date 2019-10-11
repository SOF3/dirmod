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

macro_rules! decl {
    ($name:ident: $($doc:literal)*) => {
        #[proc_macro]
        $(#[doc = $doc])*
        pub fn $name(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
            match imp::$name(ts.into()) {
                Ok(ts) => ts,
                Err(err) => err.to_compile_error(),
            }.into()
        }
    };
}

decl!(all:
      r#"Include all possible modules in the directory"#
      r#""#
      r#"# Parameters"#
      r#"The following parameter statements can be joined by semicolons."#
      r#"- `default $vis`: All modules have `$vis` visibility by default,"#
      r#"  where `$vis` can be `pub`, `pub(crate)`, etc."#
      r#"- `$vis $name1, $name2, ...`: The specified modules have `$vis`"#
      r#"  visibility, different from the default visibility."#
      r#"  Note that `priv` can also be used as a visibility here."#
      r#"- `except $name1 $name2 ...`: The specified modules are excluded."#
      r#""#
      r#"For simplicity, there is no special syntax to add doc comments."#
      r#"To document modules, either use the `//!` inner documentation syntax,"#
      r#"or use `except` to exclude declaration and declare them separately"#
      r#"from the macro call."#
      r#""#
      r#"# Example"#
      r#"```ignore"#
      r#"all!();"#
      r#"```"#
      r#""#
      r#"```ignore"#
      r#"all!(default pub(crate); pub foo);"#
      r#"```"#
      r#""#
      r#"```ignore"#
      r#"all! {"#
      r#"    default pub(super);"#
      r#"    pub(crate) foo, bar;"#
      r#"    pub qux, corge;"#
      r#"    priv lorem;"#
      r#"    except ipsum;"#
      r#"};"#
      r#"```"#
      );

mod imp;
mod parse;
