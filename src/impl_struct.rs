use syn;
use proc_macro2::TokenStream;
use quote::ToTokens;

use ::syn_utils::{
    FieldImpl,
    impl_parts_for_fields
};

pub fn impl_inspect(
    name: &syn::Ident,
    a_struct: &syn::DataStruct,
    token_impl: &mut TokenStream,
    token_impl_mut: &mut TokenStream
) {
    let fields = &a_struct.fields;

    if fields.iter().count() == 0 {
        return
    }

    let FieldImpl { field_ref, field_ref_mut, field_usage, field_usage_mut } =
        impl_parts_for_fields(fields);

    match fields {
        &syn::Fields::Named(_) => {
            (quote! { &#name { #field_ref } => { #field_usage } })
                .to_tokens(token_impl);
            (quote! { &mut #name { #field_ref_mut } => { #field_usage_mut } })
                .to_tokens(token_impl_mut);
        },
        &syn::Fields::Unnamed(_) => {
            (quote! { & #name ( #field_ref ) => { #field_usage } })
                .to_tokens(token_impl);
            (quote! { &mut #name ( #field_ref_mut ) => { #field_usage_mut } })
                .to_tokens(token_impl_mut);
        },
        &syn::Fields::Unit => ()
    }
}

// pub fn impl_field_inspect(field: &syn::Field, idx: usize) -> Option<(impl ToTokens, impl ToTokens)> {
//     let name = field.ident.as_ref()
//         .map(Clone::clone)
//         .unwrap_or_else(|| syn::Ident::new(&format!("{}", idx), Span::call_site()));

//     if is_skipped(&field.attrs) {
//         None
//     } else if let Some((use_func, use_func_mut)) = use_alternate_function(&field.attrs) {
//         let tokens =
//             quote! {
//                 #use_func(&self.#name, visitor);
//             };
//         let tokens_mut =
//             quote! {
//                 #use_func_mut(&mut self.#name, visitor);
//             };

//         Some((tokens, tokens_mut))
//     } else {
//         let tokens =
//             quote! {
//                 self.#name.inspect_resources(visitor);
//             };
//         let tokens_mut =
//             quote! {
//                 self.#name.inspect_resources_mut(visitor);
//             };

//         Some((tokens, tokens_mut))
//     }
// }



