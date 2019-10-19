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

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::token;

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

#[cfg(test)]
mod tests {
    use matches::assert_matches;
    use quote::quote;

    use crate::parse::*;

    #[test]
    fn test_priv_vis() {
        let pv: PrivVis = syn::parse2(quote!(priv)).unwrap();
        assert_matches!(pv, PrivVis::Priv(_));
        assert_token_eq(quote!(#pv), quote!());

        let pv: PrivVis = syn::parse2(quote!()).unwrap();
        assert_matches!(pv, PrivVis::Vis(syn::Visibility::Inherited));
        assert_token_eq(quote!(#pv), quote!());

        let pv: PrivVis = syn::parse2(quote!(pub)).unwrap();
        assert_matches!(pv, PrivVis::Vis(syn::Visibility::Public(_)));
        assert_token_eq(quote!(#pv), quote!(pub));

        let pv: PrivVis = syn::parse2(quote!(pub(crate))).unwrap();
        assert_matches!(pv, PrivVis::Vis(syn::Visibility::Restricted(_))); // not Visibility::Crate???
        assert_token_eq(quote!(#pv), quote!(pub(crate)));

        let pv: PrivVis = syn::parse2(quote!(pub(self))).unwrap();
        assert_matches!(pv, PrivVis::Vis(syn::Visibility::Restricted(_)));
        assert_token_eq(quote!(#pv), quote!(pub(self)));

        let pv: PrivVis = syn::parse2(quote!(pub(super))).unwrap();
        assert_matches!(pv, PrivVis::Vis(syn::Visibility::Restricted(_)));
        assert_token_eq(quote!(#pv), quote!(pub(super)));
    }
}
