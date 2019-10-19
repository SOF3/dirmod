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

use proc_macro2::Span;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::token;

use super::PrivVis;

/// A combination of `PrivVis` + an optional `use`
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

#[cfg(test)]
mod tests {
    use matches::assert_matches;
    use quote::quote;

    use crate::parse::*;

    #[test]
    fn test_parse_modifier() {
        let modifier: Modifier = syn::parse2(quote!(priv)).unwrap();
        assert_matches!(modifier.vis, PrivVis::Priv(_));
        assert!(modifier.imports.is_none());

        let modifier: Modifier = syn::parse2(quote!(priv use)).unwrap();
        assert_matches!(modifier.vis, PrivVis::Priv(_));
        assert!(modifier.imports.is_some());

        let modifier: Modifier = syn::parse2(quote!()).unwrap();
        assert_matches!(modifier.vis, PrivVis::Vis(syn::Visibility::Inherited));
        assert!(modifier.imports.is_none());

        let modifier: Modifier = syn::parse2(quote!(use)).unwrap();
        assert_matches!(modifier.vis, PrivVis::Vis(syn::Visibility::Inherited));
        assert!(modifier.imports.is_some());

        let modifier: Modifier = syn::parse2(quote!(pub(self))).unwrap();
        assert_matches!(modifier.vis, PrivVis::Vis(syn::Visibility::Restricted(_)));
        assert!(modifier.imports.is_none());

        let modifier: Modifier = syn::parse2(quote!(pub(self) use)).unwrap();
        assert_matches!(modifier.vis, PrivVis::Vis(syn::Visibility::Restricted(_)));
        assert!(modifier.imports.is_some());
    }
}
