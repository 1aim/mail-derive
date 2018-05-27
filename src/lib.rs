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

#[proc_macro_derive(InspectEmbeddedResources)]
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
            fn inspect_resources(&self, visitor: &mut impl FnMut(&Embedded)) {
                #tokens_impl
            }
            fn inspect_resources_mut(&mut self, visitor: &mut impl FnMut(&mut Embedded)) {
                #tokens_impl_mut
            }
        }
    }
}


fn impl_inspect(name: &syn::Ident, data: &syn::Data) -> (impl ToTokens, impl ToTokens) {
    let mut token_impl = TokenStream::empty();
    let mut token_impl_mut = TokenStream::empty();

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