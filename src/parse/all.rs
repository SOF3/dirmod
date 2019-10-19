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

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token;
use syn::Result;

use super::{kw, ArgDefaultVis, ArgExcept, ArgSpecialVis};

#[derive(Clone, Debug)]
pub struct Args(pub Punctuated<Arg, token::Semi>);

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Args(Punctuated::parse_terminated(input)?))
    }
}

#[derive(Clone, Debug)]
pub enum Arg {
    DefaultVis(ArgDefaultVis),
    SpecialVis(ArgSpecialVis),
    Except(ArgExcept),
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        let ret = if input.peek(kw::default) {
            Arg::DefaultVis(ArgDefaultVis {
                default: input.parse()?,
                modifier: input.parse()?,
            })
        } else if input.peek(token::Priv) || input.peek(token::Pub) {
            Arg::SpecialVis(ArgSpecialVis {
                modifier: input.parse()?,
                idents: Punctuated::parse_terminated(input)?,
            })
        } else if input.peek(kw::except) {
            Arg::Except(ArgExcept {
                except: input.parse()?,
                idents: Punctuated::parse_terminated(input)?,
            })
        } else {
            Err(input.error("invalid argument for all!()"))?
        };
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use matches::assert_matches;
    use quote::quote;

    use crate::parse::*;

    #[test]
    fn test_all_args() {
        use all::{Arg, Args};

        let args: Args = syn::parse2(quote! {
            default pub(crate);
            pub use foo, bar;
            // priv qux;
            // except corge, grault;
        })
        .unwrap();
        let mut args = args.0.into_iter();

        if let Some(Arg::DefaultVis(dv)) = args.next() {
            assert_matches!(
                dv.modifier.vis,
                PrivVis::Vis(syn::Visibility::Restricted(_))
            );
            assert!(dv.modifier.imports.is_none());
        } else {
            panic!("assertion failed: args.next() matches Some(Arg::DefaultVis(_))");
        }

        if let Some(Arg::SpecialVis(sv)) = args.next() {
            assert_matches!(sv.modifier.vis, PrivVis::Vis(syn::Visibility::Public(_)));
            assert!(sv.modifier.imports.is_some());
        } else {
            panic!("assertion failed: args.next() matches Some(Arg::SpecialVis(_))");
        }

        if let Some(Arg::SpecialVis(sv)) = args.next() {
            assert_matches!(sv.modifier.vis, PrivVis::Priv(_));
            assert!(sv.modifier.imports.is_none());
        } else {
            panic!("assertion failed: args.next() matches Some(Arg::SpecialVis(_))");
        }

        if let Some(Arg::Except(except)) = args.next() {
            assert_eq!(
                except
                    .idents
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
                vec!["corge", "grault"]
            );
        } else {
            panic!("assertion failed: args.next() matches Some(Arg::Except(_))");
        }
    }
}
