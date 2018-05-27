#![recursion_limit = "128"]

extern crate proc_macro2;
extern crate proc_macro as rust;
extern crate syn;
#[macro_use]
extern crate quote;

mod impl_struct;
mod impl_enum;
mod syn_utils;

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



#[cfg(test)]
mod test {
    use std::str::FromStr;
    use syn;
    use proc_macro2::TokenStream;
    use super::*;


    macro_rules! lines {
        ($($line:expr),*) => (concat!($($line, "\n"),*));
    }
    //TODO make integration tests load files etc.
    #[test]
    fn test_struct_thingy() {
        let input = lines!(
            "struct A {",
            "    #[mail(inspect_skip)]",
            "    f1: u32,",
            "    fa: f64,",
            "    #[mail(inspect_with=\"(la::special, la::special_mut)\")]",
            "    fe: i32,",
            "    f2: Resource",
            "}"
        );

        let expected = lines!(
            "impl InspectEmbeddedResources for A {",
            "    fn inspect_resources(&self, visitor: &mut impl FnMut(&Embedded)) {",
            "        #[allow(unused_variables)]",
            "        match self {",
            "            &A { ref f1, ref fa, ref fe, ref f2, } => {",
            "                fa.inspect_resources(visitor);",
            "                la::special(fe, visitor);",
            "                f2.inspect_resources(visitor);",
            "            }",
            "        }",
            "    }",
            "    fn inspect_resources_mut(&mut self, visitor: &mut impl FnMut (&mut Embedded)) {",
            "        #[allow(unused_variables)]",
            "        match self {",
            "            &mut A { ref mut f1, ref mut fa, ref mut fe, ref mut f2, } => {",
            "                fa.inspect_resources_mut(visitor);",
            "                la::special_mut(fe, visitor);",
            "                f2.inspect_resources_mut(visitor);",
            "            }",
            "        }",
            "    }",
            "}"
        );

        let input_tokens = TokenStream::from_str(input).unwrap();
        let buffer = syn::buffer::TokenBuffer::new2(input_tokens);
        let (ref parsed, _) = <syn::DeriveInput as syn::synom::Synom>::parse(buffer.begin()).unwrap();
        let result = impl_inspect_embedded_resource(parsed);
        let got_string = format!("{}", result.into_token_stream());

        let expected_tokens =TokenStream::from_str(expected).unwrap();
        let expected_string = format!("{}", expected_tokens.into_token_stream());

        assert_eq!(got_string, expected_string);
    }


    #[test]
    fn test_struct_thingy2() {
        let input = lines!(
            "struct Au(u32, u32);"
        );

        let expected = lines!(
            "impl InspectEmbeddedResources for Au {",
            "    fn inspect_resources(&self, visitor: &mut impl FnMut(&Embedded)) {",
            "        #[allow(unused_variables)]",
            "        match self {",
            "            &Au(ref f0, ref f1, ) => {",
            "                f0.inspect_resources(visitor);",
            "                f1.inspect_resources(visitor);",
            "            }",
            "        }",
            "    }",
            "    fn inspect_resources_mut(&mut self, visitor: &mut impl FnMut (&mut Embedded)) {",
            "        #[allow(unused_variables)]",
            "        match self {",
            "            &mut Au(ref mut f0, ref mut f1, ) => {",
            "                f0.inspect_resources_mut(visitor);",
            "                f1.inspect_resources_mut(visitor);",
            "            }",
            "        }",
            "    }",
            "}"
        );

        let input_tokens = TokenStream::from_str(input).unwrap();
        let buffer = syn::buffer::TokenBuffer::new2(input_tokens);
        let (ref parsed, _) = <syn::DeriveInput as syn::synom::Synom>::parse(buffer.begin()).unwrap();
        let result = impl_inspect_embedded_resource(parsed);
        let got_string = format!("{}", result.into_token_stream());

        let expected_tokens =TokenStream::from_str(expected).unwrap();
        let expected_string = format!("{}", expected_tokens.into_token_stream());

        assert_eq!(got_string, expected_string);
    }

    #[test]
    fn test_struct_thingy3() {
        let input = lines!(
            "struct A;"
        );

        let expected = lines!(
            "impl InspectEmbeddedResources for A {",
            "    fn inspect_resources(&self, visitor: &mut impl FnMut(&Embedded)) {",
            "    }",
            "    fn inspect_resources_mut(&mut self, visitor: &mut impl FnMut (&mut Embedded)) {",
            "    }",
            "}"
        );

        let input_tokens = TokenStream::from_str(input).unwrap();
        let buffer = syn::buffer::TokenBuffer::new2(input_tokens);
        let (ref parsed, _) = <syn::DeriveInput as syn::synom::Synom>::parse(buffer.begin()).unwrap();
        let result = impl_inspect_embedded_resource(parsed);
        let got_string = format!("{}", result.into_token_stream());

        let expected_tokens =TokenStream::from_str(expected).unwrap();
        let expected_string = format!("{}", expected_tokens.into_token_stream());

        assert_eq!(got_string, expected_string);
    }

    #[test]
    fn test_enum_thingy() {
        let input = lines!(
            "enum A {",
            "    VariA,",
            "    VariB(u32, u32, #[mail(inspect_skip)] u8),",
            "    VariC {",
            "        f1: u32,",
            "        #[mail(inspect_with=\"(afn, afn_mut)\")]",
            "        f2: u32",
            "    }",
            "}"
        );

        let expected = lines!(
            "impl InspectEmbeddedResources for A {",
            "    fn inspect_resources(&self, visitor: &mut impl FnMut(&Embedded)) {",
            "        #[allow(unused_variables)]",
            "        match self {",
            "            &A::VariB(ref f0, ref f1, ref f2, ) => {",
            "                f0.inspect_resources(visitor);",
            "                f1.inspect_resources(visitor);",
            "            },",
            "            &A::VariC { ref f1, ref f2, } => {",
            "                f1.inspect_resources(visitor);",
            "                afn(f2, visitor);",
            "            },",
            "        }",
            "    }",
            "    fn inspect_resources_mut(&mut self, visitor: &mut impl FnMut(&mut Embedded)) {",
            "        #[allow(unused_variables)]",
            "        match self {",
            "            &mut A::VariB(ref mut f0, ref mut f1, ref mut f2, ) => {",
            "                f0.inspect_resources_mut(visitor);",
            "                f1.inspect_resources_mut(visitor);",
            "            },",
            "            &mut A::VariC { ref mut f1, ref mut f2, } => {",
            "                f1.inspect_resources_mut(visitor);",
            "                afn_mut(f2, visitor);",
            "            },",
            "        }",
            "    }",
            "}"
        );

        let input_tokens = TokenStream::from_str(input).unwrap();
        let buffer = syn::buffer::TokenBuffer::new2(input_tokens);
        let (ref parsed, _) = <syn::DeriveInput as syn::synom::Synom>::parse(buffer.begin()).unwrap();
        let result = impl_inspect_embedded_resource(parsed);
        let got_string = format!("{}", result.into_token_stream());

        let expected_tokens =TokenStream::from_str(expected).unwrap();
        let expected_string = format!("{}", expected_tokens.into_token_stream());

        assert_eq!(got_string, expected_string);
    }
}