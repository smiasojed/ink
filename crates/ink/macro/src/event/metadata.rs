// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use proc_macro2::TokenStream as TokenStream2;
use quote::{
    quote,
    quote_spanned,
};
use syn::spanned::Spanned;

/// Derives the `ink::Event` trait for the given `struct`.
pub fn event_metadata_derive(mut s: synstructure::Structure) -> TokenStream2 {
    s.bind_with(|_| synstructure::BindStyle::Move)
        .add_bounds(synstructure::AddBounds::Fields)
        .underscore_const(true);
    match &s.ast().data {
        syn::Data::Struct(_) => event_metadata_derive_struct(s),
        _ => {
            panic!("can only derive `EventMetadata` for Rust `struct` items")
        }
    }
}

/// `Event` derive implementation for `struct` types.
fn event_metadata_derive_struct(mut s: synstructure::Structure) -> TokenStream2 {
    assert_eq!(s.variants().len(), 1, "can only operate on structs");
    let span = s.ast().span();

    let variant = &s.variants()[0];
    let ident = variant.ast().ident;

    s.bound_impl(
        quote!(::ink::metadata::EventMetadata),
        quote! {
           fn event_spec() -> ::ink::metadata::EventSpec {
               // register this event metadata function in the distributed slice for combining all
               // events referenced in the contract binary.
               #[::ink::metadata::linkme::distributed_slice(::ink::metadata::EVENTS)]
               #[linkme(crate = ::ink::metadata::linkme)]
               static EVENT_METADATA: fn() -> ::ink::metadata::EventSpec =
                   <#ident as ::ink::metadata::EventMetadata>::event_spec;

               // todo: check that cfg attributes work here
               ::ink::metadata::EventSpec::new(::core::stringify!(#ident))
                       // todo: add signanture topic if not anonymous
                       // todo: add fields, with topic flag.
                       .args([
                           // #( #args ),*
                       ])
                       // todo: add docs
                       .docs([
                           // #( #docs ),*
                       ])
                       .done()
           }
        },
    )
}