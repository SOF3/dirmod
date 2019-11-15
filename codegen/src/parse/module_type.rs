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

use matches::matches;
use syn::parse::{Parse, ParseStream, Result};

use super::kw;

#[derive(Clone, Debug)]
pub enum ModuleTypeKw {
    File(kw::file),
    Dir(kw::dir),
    All,
}

impl ModuleTypeKw {
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_) | Self::All)
    }
    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(_) | Self::All)
    }
}

impl Parse for ModuleTypeKw {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(kw::file) {
            Ok(Self::File(input.parse()?))
        } else if input.peek(kw::file) {
            Ok(Self::Dir(input.parse()?))
        } else {
            Ok(Self::All)
        }
    }
}
