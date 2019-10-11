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

use proc_macro2::{Span, TokenStream};
use quote::quote;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token;
use syn::{Ident, Result};

pub mod all {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Args(pub Punctuated<Arg, token::Semi>);

    impl Parse for Args {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(Args(input.parse_terminated(Arg::parse)?))
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
                    idents: input.parse_terminated(Ident::parse)?,
                })
            } else if input.peek(kw::except) {
                Arg::Except(ArgExcept {
                    except: input.parse()?,
                    idents: input.parse_terminated(Ident::parse)?,
                })
            } else {
                Err(input.error("invalid argument for all!()"))?
            };
            Ok(ret)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Modifier {
    pub vis: PrivVis,
    pub imports: Option<token::Use>,
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            vis: input.parse()?,
            imports: if input.peek(token::Use) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

impl Spanned for Modifier {
    fn span(&self) -> Span {
        self.vis.span()
    }
}

#[derive(Clone, Debug)]
pub enum PrivVis {
    Priv(token::Priv),
    Vis(syn::Visibility),
}

impl Parse for PrivVis {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(token::Priv) {
            Ok(Self::Priv(input.parse()?))
        } else {
            Ok(Self::Vis(input.parse()?))
        }
    }
}

impl ToTokens for PrivVis {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            Self::Priv(_) => quote!(),
            Self::Vis(vis) => quote!(#vis),
        };
        q.to_tokens(tokens)
    }
}

#[derive(Clone, Debug)]
pub struct ArgDefaultVis {
    default: kw::default,
    pub modifier: Modifier,
}

impl Spanned for ArgDefaultVis {
    fn span(&self) -> Span {
        self.default.span()
    }
}

#[derive(Clone, Debug)]
pub struct ArgSpecialVis {
    pub modifier: Modifier,
    pub idents: Punctuated<Ident, token::Comma>,
}

impl Spanned for ArgSpecialVis {
    fn span(&self) -> Span {
        self.modifier.span()
    }
}

#[derive(Clone, Debug)]
pub struct ArgExcept {
    except: kw::except,
    pub idents: Punctuated<Ident, token::Comma>,
}

impl Spanned for ArgExcept {
    fn span(&self) -> Span {
        self.except.span()
    }
}

mod kw {
    syn::custom_keyword!(default);
    syn::custom_keyword!(except);
}
