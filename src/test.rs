//! As proc_macro crates can't export this the tests are here instead of in `"./tests"
// TODO: See if custom targets can be used to have tests in tests (i.e. have a different
//       target which is not a proc_macro).
// TODO: Maybe use custom dynamic test runner.

use std::path::Path;
use std::str::FromStr;
use std::fs;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn;

use super::impl_inspect_embedded_resource;

const TEST_BASE_DIR: &str = "./test_resources/inspect_embedded_resources/";

fn run_e2e_test(path: impl AsRef<Path>) {
    let file_base = Path::new(TEST_BASE_DIR).join(path.as_ref());

    let base_path = file_base.with_extension("type.rs");
    let impl_path = file_base.with_extension("impl.rs");

    let input_tokens = read_tokens_from_file(base_path);
    let parsed = derive_input_from_tokens(input_tokens);
    let got = impl_inspect_embedded_resource(&parsed);
    let got_string = format!("{}", got.into_token_stream());

    let expected_tokens = read_tokens_from_file(impl_path);
    let expected_string = format!("{}", expected_tokens);

    assert_eq!(got_string, expected_string);
}

fn read_tokens_from_file(file: impl AsRef<Path>) -> TokenStream {
    let string = fs::read_to_string(file).unwrap();
    TokenStream::from_str(&string).unwrap()
}

fn derive_input_from_tokens(tokens: TokenStream) -> syn::DeriveInput {
    let buffer = syn::buffer::TokenBuffer::new2(tokens);
    let (parsed, _) = <syn::DeriveInput as syn::synom::Synom>::parse(buffer.begin()).unwrap();
    parsed
}

macro_rules! impl_test {
    ($($name:ident),*) => ($(
        #[test]
        fn $name() {
            run_e2e_test(Path::new(stringify!($name)));
        }
    )*);
}

impl_test! {
    unit_struct,
    tuple_struct,
    struct_struct_complex,
    enum_complex
}
