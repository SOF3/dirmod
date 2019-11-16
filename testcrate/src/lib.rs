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

dirmod::all!(except corge);

#[test]
pub fn test() {
    assert_eq!(foo::FOO, 1);
    assert_eq!(bar::BAR, 3);
    assert_eq!(qux::QUX, if cfg!(target_family = "unix") {
        "NIX"
    } else {
        "WIN"
    });
}
