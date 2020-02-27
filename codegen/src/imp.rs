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

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use smallvec::SmallVec;
use syn::spanned::Spanned;
use syn::{Error, Result};

use crate::parse;

macro_rules! parse_args {
    ($ts:expr, $mod:ident;
        single: $($svar:ident),*;
        multi: $($mvar:ident),*;
    ) => {{
        let args = syn::parse2::<parse::$mod::Args>($ts).map_err(crate::context("argument parsing"))?;
        let single = ($({
            #[allow(irrefutable_let_patterns)]
            let rep = args.0.iter().filter_map(|arg| {
                if let parse::$mod::Arg::$svar(arg) = arg {
                    Some(arg.clone())
                } else {
                    None
                }
            }).collect::<SmallVec<[_; 1]>>();
            if rep.len() > 1 {
                return Err(Error::new(rep[1].span(), "The statement is repeated"));
            }
            rep.into_iter().next()
        },)*);
        let multi = ($({
            args.0.iter().filter_map(|arg| {
                if let parse::$mod::Arg::$mvar(arg) = arg {
                    Some(arg.clone())
                } else {
                    None
                }
            }).collect::<Vec<_>>()
        },)*);
        (single, multi)
    }};
}

fn apply_modifier(
    modifier: &parse::Modifier,
    ident: syn::Ident,
    meta: Option<TokenStream>,
) -> TokenStream {
    let meta = &meta.unwrap_or(quote!());
    let vis = &modifier.vis;
    if let Some(import) = modifier.imports.as_ref() {
        quote!(#meta mod #ident; #meta #vis #import #ident::*;)
    } else {
        quote!(#meta #vis mod #ident;)
    }
}

pub fn all(ts: TokenStream) -> Result<TokenStream> {
    let ((), (dv, sv, excepts)) = parse_args! {
        ts, all;
        single: ;
        multi: DefaultVis, SpecialVis, Except;
    };

    let mut default_file = None;
    let mut default_dir = None;
    for dve in dv {
        if dve.module_type.is_file() && default_file.is_none() {
            default_file = Some(dve.modifier.clone());
        }
        if dve.module_type.is_dir() && default_dir.is_none() {
            default_dir = Some(dve.modifier.clone());
        }
    }

    let default_file = default_file.unwrap_or_else(parse::Modifier::default_file);
    let default_dir = default_dir.unwrap_or_else(parse::Modifier::default_dir);

    let mut special = HashMap::<String, (Span, Rc<parse::Modifier>)>::new();
    for sve in sv {
        let modifier = Rc::new(sve.modifier.clone());
        let span = sve.modifier.span();
        for name_ident in sve.idents {
            let name = name_ident.to_string();
            if special.contains_key(&name) {
                return Err(Error::new(
                    name_ident.span(),
                    "The module has multiple visibilities",
                ));
            }
            special.insert(name, (span, modifier.clone()));
        }
    }

    let except = excepts
        .into_iter()
        .flat_map(|except| except.idents.into_iter())
        .map(|ident| ident.to_string())
        .collect::<HashSet<_>>();

    let mods = list_mods()?
        .iter()
        .map(|(name, ty)| -> Result<TokenStream> {
            if except.contains(name) {
                if special.contains_key(name) {
                    Err(Error::new(
                        special[name].0,
                        "The module has a special visibility but is also excluded in `except`",
                    ))
                } else {
                    Ok(quote!())
                }
            } else {
                let ni = syn::Ident::new(name, Span::call_site());
                let modifier = special.get(name).map_or_else(
                    || match ty {
                        ModuleType::File => &default_file,
                        ModuleType::Dir => &default_dir,
                    },
                    |(_, modifier)| &modifier,
                );
                let stmt = apply_modifier(modifier, ni, None);
                Ok(stmt)
            }
        })
        .collect::<Result<Vec<TokenStream>>>()?;

    let q = quote!(#(#mods)*);
    Ok(q)
}

pub fn os(ts: TokenStream) -> Result<TokenStream> {
    cfg(ts, "target_os")
}

pub fn family(ts: TokenStream) -> Result<TokenStream> {
    cfg(ts, "target_family")
}

pub fn feature(ts: TokenStream) -> Result<TokenStream> {
    cfg(ts, "feature")
}

fn cfg(ts: TokenStream, flag_name: &str) -> Result<TokenStream> {
    let flag = syn::Ident::new(flag_name, Span::call_site());

    let ((arg,), ()) = parse_args! {
        ts, cfg;
        single: Cfg;
        multi: ;
    };

    let mods = list_mods()
        .map_err(crate::context("directory listing"))?
        .into_iter()
        .map(|(name, _)| name)
        .collect::<Vec<_>>();
    let mods_code = mods.iter().map(|name| {
        apply_modifier(
            &arg.as_ref()
                .map_or_else(parse::Modifier::default_cfg, |arg| arg.modifier.clone()),
            syn::Ident::new(name, Span::call_site()),
            Some(quote! ( #[cfg(#flag = #name)] )),
        )
    });
    let el = if let Some(Some((_, error))) = arg.as_ref().map(|arg| &arg.error) {
        let error = error.as_ref().map_or(
            format!("{} must be one of \"{}\"", flag, mods.join("\", \"")),
            |error| error.value(),
        );
        quote! {
            #[cfg(not(any(#(#flag = #mods),*)))]
            compile_error!(#error);
        }
    } else {
        quote!()
    };

    let ret = quote! {
        #(#mods_code)*
        #el
    };
    Ok(ret)
}

fn list_mods() -> Result<Vec<(String, ModuleType)>> {
    fn me<T: std::fmt::Display>(err: T) -> Error {
        Error::new(proc_macro2::Span::call_site(), err)
    }

    macro_rules! mes {
        ($fmt:literal) => {
            |err| me(format!($fmt, err))
        };
    }

    let span = proc_macro::Span::call_site();
    let src = span.source_file();
    if !src.is_real() {
        return Err(me("dirmod can only be invoked directly, not via macros"));
    }

    let dir = src
        .path()
        .parent()
        .expect("parent directory does not exist")
        .read_dir()
        .map_err(mes!("error reading parent directory of current file: {}"))?;
    let mut ret = vec![];
    for entry in dir {
        let entry = entry.map_err(mes!("error reading dir entry: {}"))?;
        let path = entry.path();
        let ft = entry
            .file_type()
            .map_err(mes!("error checking dir entry file type: {}"))?;
        if ft.is_file()
            && path.extension().and_then(|str| str.to_str()) == Some("rs")
            && path.file_name() != src.path().file_name()
        {
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| me("Module is not UTF-8 compliant"))?;
            ret.push((name[..(name.len() - 3)].to_string(), ModuleType::File));
        } else if ft.is_dir() && path.join("mod.rs").is_file() {
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| me("Module is not UTF-8 compliant"))?;
            ret.push((name, ModuleType::Dir));
        }
    }

    Ok(ret)
}

#[derive(Clone, Debug)]
enum ModuleType {
    File,
    Dir,
}
