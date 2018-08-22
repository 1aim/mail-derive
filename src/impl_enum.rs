use syn;
use proc_macro2::TokenStream;
use quote::ToTokens;

use ::syn_utils::{
    is_skipped,
    impl_parts_for_fields,
    FieldImpl
};

pub fn impl_inspect(
    name: &syn::Ident,
    a_enum: &syn::DataEnum,
    token_impl: &mut TokenStream,
    token_impl_mut: &mut TokenStream
) {
    let mut add_default_branch = false;
    for variant in a_enum.variants.iter() {
        if let Some((new_tokens, new_tokens_mut)) = impl_variant_inspect(name, variant) {
            new_tokens.to_tokens(token_impl);
            new_tokens_mut.to_tokens(token_impl_mut);
        } else {
            add_default_branch = true;
        }
    }
    if add_default_branch {
        (quote!{ _ => {} }).to_tokens(token_impl);
        (quote!{ _ => {} }).to_tokens(token_impl_mut);
    }
}

fn impl_variant_inspect(name: &syn::Ident, variant: &syn::Variant)
    -> Option<(impl ToTokens, impl ToTokens)>
{
    let vname = &variant.ident;
    if variant.fields.iter().count() == 0 || is_skipped(&variant.attrs) {
        None
    } else {
        match &variant.fields {
            syn::Fields::Named(_) => Some(impl_variant_inspect_named(name, vname, &variant.fields)),
            syn::Fields::Unnamed(_) => Some(impl_variant_inspect_unnamed(name, vname, &variant.fields)),
            syn::Fields::Unit => None
        }
    }
}

fn impl_variant_inspect_named(name: &syn::Ident, vname: &syn::Ident, fields: &syn::Fields)
    -> (TokenStream, TokenStream)
{
    let FieldImpl { field_ref, field_ref_mut, field_usage, field_usage_mut } =
        impl_parts_for_fields(fields);

    let t = quote! {
        & #name :: #vname { #field_ref  } => {
            #field_usage
        },
    };
    let t_mut = quote! {
        &mut #name :: #vname { #field_ref_mut } => {
            #field_usage_mut
        },
    };

    (t.into_token_stream(), t_mut.into_token_stream())
}


fn impl_variant_inspect_unnamed(name: &syn::Ident, vname: &syn::Ident, fields: &syn::Fields)
    -> (TokenStream, TokenStream)
{
    let FieldImpl { field_ref, field_ref_mut, field_usage, field_usage_mut } =
        impl_parts_for_fields(fields);

    let t = quote! {
        & #name :: #vname(#field_ref) => {
            #field_usage
        },
    };
    let t_mut = quote! {
        &mut #name :: #vname(#field_ref_mut) => {
            #field_usage_mut
        },
    };

    (t.into_token_stream(), t_mut.into_token_stream())
}
