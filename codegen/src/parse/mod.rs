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

// Unfortunately I can't use dirmod here :(

pub mod all;
pub mod cfg;

mod modifier;
pub use modifier::*;

mod priv_vis;
pub use priv_vis::*;

mod arg;
pub use arg::*;

mod kw {
    syn::custom_keyword!(default);
    syn::custom_keyword!(except);
}

#[cfg(test)]
fn assert_token_eq(a: proc_macro2::TokenStream, b: proc_macro2::TokenStream) {
    use proc_macro2::TokenTree;

    for (p, q) in a.clone().into_iter().zip(b.clone().into_iter()) {
        match (p, q) {
            (TokenTree::Punct(p), TokenTree::Punct(q)) => assert_eq!(p.as_char(), q.as_char()),
            (TokenTree::Ident(p), TokenTree::Ident(q)) => assert_eq!(p.to_string(), q.to_string()),
            (TokenTree::Literal(p), TokenTree::Literal(q)) => {
                assert_eq!(p.to_string(), q.to_string())
            }
            (TokenTree::Group(p), TokenTree::Group(q)) => assert_token_eq(p.stream(), q.stream()),
            _ => panic!("assertion failed: {} != {}", a, b),
        }
    }
}
