//! This crate provides a custom derives for the mail crate.
//!
//! Currently it only contains the `InspectEmbeddedResource`
//! derive which allows deriving the `InspectEmbeddedResource`
//! trait from the `mail-template` (sub-)crate.
//!
//! # InspectEmbeddedResource
//!
//! Implements `InspectEmbeddedResource` by forwarding calls to it's
//! methods to all fields.
//!
//! **Belongs to**: `mail-template`
//!
//! **Applicable to**: struct, tuple struct, enum, enum with named fields
//!
//! **Attribute Scope**: `mail` (e.g. `#[mail(inspect_skip)]`)
//!
//! **Field Attributes**:
//!
//! - `inspect_skip`, don't forward calls to the annotated _field_
//! - `inspect_with = "(no_mut_fn, mut_fn)"`, specifies two function
//!   which are called with a reference to the field (1st param) and
//!   the visitor (2nd param) instead of calling inspect_resource(_mut)
//!   on the field. Note that the functions in the tuple are interpreted
//!   as paths so e.g. `::some_thing::fn1` would be potentially valid.
//!
//! (Note that field attributes apply to any kind of field wether it's
//! named or unnamed appears in a struct or enum.)
//!
//! **Type Attributes**: None
//!
//! **Enum Variant Attributes**: None
//!
//!
//!
//!
#![recursion_limit = "128"]

extern crate proc_macro2;
extern crate proc_macro as rust;
extern crate syn;
#[macro_use]
extern crate quote;

mod impl_struct;
mod impl_enum;
mod syn_utils;

#[cfg(test)]
mod test;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn_utils::is_empty_type;

const ATTR_NAME_SPACE: &str = "mail";

#[proc_macro_derive(InspectEmbeddedResources, attributes(mail))]
pub fn inspect_embedded_resource_derive(input: rust::TokenStream) -> rust::TokenStream {
    let input = syn::parse(input).unwrap();
    let new_tokens = impl_inspect_embedded_resource(&input);
    new_tokens.into_token_stream().into()
}

fn impl_inspect_embedded_resource(ast: &syn::DeriveInput) -> impl ToTokens + Sized {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let (tokens_impl, tokens_impl_mut) = impl_inspect(name, &ast.data);

    quote! {
        impl #impl_generics InspectEmbeddedResources for #name #ty_generics
            #where_clause
        {
            fn inspect_resources(&self, visitor: &mut FnMut(&Embedded)) {
                #tokens_impl
            }
            fn inspect_resources_mut(&mut self, visitor: &mut FnMut(&mut Embedded)) {
                #tokens_impl_mut
            }
        }
    }
}


fn impl_inspect(name: &syn::Ident, data: &syn::Data) -> (impl ToTokens, impl ToTokens) {
    let mut token_impl = TokenStream::new();
    let mut token_impl_mut = TokenStream::new();

    if is_empty_type(data) {
        return (quote! {}, quote! {})
    }

    match data {
        &syn::Data::Struct(ref a_struct) => {
            impl_struct::impl_inspect(name, a_struct, &mut token_impl, &mut token_impl_mut)
        },
        &syn::Data::Enum(ref a_enum) => {
            impl_enum::impl_inspect(name, a_enum, &mut token_impl, &mut token_impl_mut)
        },
        _ => panic!("[derive(InspectEmbeddedResources)] can only derive struct's and enums")
    }

    (quote! {
        #[allow(unused_variables)]
        match self {
            #token_impl
        }
    }, quote! {
        #[allow(unused_variables)]
        match self {
            #token_impl_mut
        }
    })
}
