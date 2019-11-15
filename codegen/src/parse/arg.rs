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

use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token;

use super::{kw, Modifier, ModuleTypeKw};

#[derive(Clone, Debug)]
pub struct ArgDefaultVis {
    pub(super) default: kw::default,
    pub module_type: ModuleTypeKw,
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
    pub(super) except: kw::except,
    pub idents: Punctuated<Ident, token::Comma>,
}

impl Spanned for ArgExcept {
    fn span(&self) -> Span {
        self.except.span()
    }
}

#[derive(Clone, Debug)]
pub struct ArgCfg {
    pub modifier: Modifier,
    pub error: Option<(token::OrOr, Option<syn::LitStr>)>,
}

impl Parse for ArgCfg {
    fn parse(input: ParseStream) -> Result<Self> {
        let modifier = input.parse()?;
        let error = if input.peek(token::OrOr) {
            let or_or = input.parse()?;
            let msg = if input.is_empty() {
                None
            } else {
                input.parse()?
            };
            Some((or_or, msg))
        } else {
            None
        };

        Ok(Self { modifier, error })
    }
}

impl Spanned for ArgCfg {
    fn span(&self) -> Span {
        self.modifier.span()
    }
}

#[cfg(test)]
mod tests {
    use matches::assert_matches;
    use quote::quote;

    use crate::parse::*;

    #[test]
    fn test_arg_default_vis() {
        let arg: all::Arg = syn::parse2(quote!(default pub(crate))).unwrap();
        let dv = if let all::Arg::DefaultVis(dv) = arg {
            dv
        } else {
            panic!("assertion failed: arg matches Arg::DefaultVis");
        };
        assert_matches!(
            dv.modifier.vis,
            PrivVis::Vis(syn::Visibility::Restricted(_))
        );
        assert!(dv.modifier.imports.is_none());
    }

    #[test]
    fn test_arg_special_vis() {
        let arg: all::Arg = syn::parse2(quote!(pub use foo)).unwrap();
        let sv = if let all::Arg::SpecialVis(sv) = arg {
            sv
        } else {
            panic!("assertion failed: arg matches Arg::SpecialVis(_)")
        };
        assert_matches!(sv.modifier.vis, PrivVis::Vis(syn::Visibility::Public(_)));
        assert!(sv.modifier.imports.is_some());
        assert_eq!(
            sv.idents
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            vec!["foo".to_string()]
        );

        let arg: all::Arg = syn::parse2(quote!(priv foo, bar)).unwrap();
        let sv = if let all::Arg::SpecialVis(sv) = arg {
            sv
        } else {
            panic!("assertion failed: arg matches Arg::SpecialVis(_)")
        };
        assert_matches!(sv.modifier.vis, PrivVis::Priv(_));
        assert!(sv.modifier.imports.is_none());
        assert_eq!(
            sv.idents
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            vec!["foo".to_string(), "bar".to_string()]
        );
    }

    #[test]
    fn test_arg_except() {
        let arg: all::Arg = syn::parse2(quote!(except corge, grault)).unwrap();
        let ex = if let all::Arg::Except(ex) = arg {
            ex
        } else {
            panic!("assertion failed: arg matches Arg::Except(_)")
        };

        assert_eq!(
            ex.idents
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            vec!["corge", "grault"]
        );
    }
}
