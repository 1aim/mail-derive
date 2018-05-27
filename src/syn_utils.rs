use syn;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;

pub fn check_mail_attr<R>(
    attrs: &Vec<syn::Attribute>,
    func: impl FnMut(&syn::MetaList) -> R
) -> Vec<R> {
    let mut func = func;
    let mut out = Vec::with_capacity(attrs.len());
    for attr in attrs {
        if let Some(syn::Meta::List(list)) = attr.interpret_meta() {
            if list.ident == ::ATTR_NAME_SPACE {
                out.push(func(&list))
            }
        }
    }
    out
}

// pub fn function_from_lit(lit: &syn::Lit) -> syn::ExprPath {
//     if let &syn::Lit::Str(ref str_lit) = lit {
//         str_lit.parse::<syn::ExprPath>().unwrap()
//     } else {
//         panic!(format!("`{:?}` is not a string literal usable for an function",
//             lit.clone().into_token_stream()))
//     }
// }


pub fn two_functions_from_lit(lit: &syn::Lit) -> (syn::ExprPath, syn::ExprPath) {
    if let &syn::Lit::Str(ref str_lit) = lit {
        let tuple = str_lit.parse::<syn::ExprTuple>().unwrap();
        if tuple.elems.len() != 2 {
            panic!("string literals is not of the form \"(some::function1, function2)\"");
        }

        let first =
            if let syn::Expr::Path(ref path) = tuple.elems.first().unwrap().value() {
                path.clone()
            } else {
                panic!("expr in tuple has to be a path but isn't")
            };

        let last =
            if let syn::Expr::Path(ref path) = tuple.elems.last().unwrap().value() {
                path.clone()
            } else {
                panic!("expr in tuple has to be a path but isn't")
            };

        (first, last)
    } else {
        panic!(format!("`{:?}` is not a string literal usable for two functions",
            lit.clone().into_token_stream()))
    }
}

pub fn mlist_len_1_nested_meta<R>(
    mlist: &syn::MetaList,
    func: impl FnOnce(&syn::Meta) -> Option<R>,
    len_error: &'static str
) -> Option<R>
{
    let nested_len = mlist.nested.len();
    if let Some(first) = mlist.nested.first() {
        let res =
            match first.value() {
                &syn::NestedMeta::Meta(ref meta) => {
                    func(meta)
                },
                _ => None
            };

        if res.is_some() && nested_len > 1 {
            panic!(len_error);
        }

        res
    } else {
        None
    }
}

pub fn meta_list_is_inspect_skip(mlist: &syn::MetaList) -> bool {
    const ACTION: &str = "inspect_skip";
    const ERR_MSG: &str = "malformed #[mail(inspect_skip)]";

    let res =
        mlist_len_1_nested_meta(mlist, |meta| {
            match meta {
                &syn::Meta::Word(ref ident) if ident == ACTION => Some(()),
                other if other.name() == ACTION => panic!(ERR_MSG),
                _ => None
            }
        }, ERR_MSG);

    res.is_some()
}


/// returns true if any attribute is `#[mail(inspect_skip)]`
pub fn is_skipped(attrs: &Vec<syn::Attribute>) -> bool {
    check_mail_attr(attrs, meta_list_is_inspect_skip)
        .into_iter()
        .any(|v| v)
}

/// returns a tuple of two paths/functions if a `#[mail(inspect_with="(path1, path2)")]` is given
pub fn use_alternate_function(attrs: &Vec<syn::Attribute>)
    -> Option<(syn::ExprPath, syn::ExprPath)>
{
    const ACTION: &str = "inspect_with";
    const ERR_MSG: &str = "malformed #[mail(inspect_with =\"func\")]";

    let results =
        check_mail_attr(attrs, |mlist| {
            mlist_len_1_nested_meta(mlist, |meta| match meta {
                &syn::Meta::NameValue(ref nv) if nv.ident == ACTION => {
                    Some(two_functions_from_lit(&nv.lit))
                },
                other if other.name() == ACTION => panic!(ERR_MSG),
                _ => None
            }, ERR_MSG)
        });

    let mut end_result = None;

    for res in results.into_iter() {
        if res.is_some() {
            if end_result.is_none() {
                end_result = res;
            } else {
                panic!("multiple #[mail(inspect_with =\"func\")]")
            }
        }
    }

    end_result
}


pub struct FieldImpl {
    pub field_ref: TokenStream,
    pub field_ref_mut: TokenStream,
    pub field_usage: TokenStream,
    pub field_usage_mut: TokenStream
}


pub fn impl_parts_for_fields(fields: &syn::Fields) -> FieldImpl {
    let mut field_ref = TokenStream::empty();
    let mut field_ref_mut = TokenStream::empty();
    let mut field_usage = TokenStream::empty();
    let mut field_usage_mut = TokenStream::empty();

    for (idx, field) in fields.iter().enumerate() {
        let fname = field.ident.as_ref()
            .map(Clone::clone)
            .unwrap_or_else(|| syn::Ident::new(&format!("f{}", idx), Span::call_site()));


        (quote! {
            ref #fname,
        }).to_tokens(&mut field_ref);

        (quote! {
            ref mut #fname,
        }).to_tokens(&mut field_ref_mut);

        if !is_skipped(&field.attrs) {
            if let Some((un, un_mut)) = use_alternate_function(&field.attrs) {
                (quote! {
                    #un(#fname, visitor);
                }).to_tokens(&mut field_usage);
                (quote! {
                    #un_mut(#fname, visitor);
                }).to_tokens(&mut field_usage_mut);
            } else {
                (quote! {
                    #fname.inspect_resources(visitor);
                }).to_tokens(&mut field_usage);
                (quote! {
                    #fname.inspect_resources_mut(visitor);
                }).to_tokens(&mut field_usage_mut);
            }

        }
    }


    FieldImpl {
        field_ref, field_ref_mut,
        field_usage, field_usage_mut
    }
}

pub fn is_empty_type(data: &syn::Data) -> bool {
    let count = match data {
        &syn::Data::Struct(ref a_struct) => a_struct.fields.iter().count(),
        &syn::Data::Enum(ref a_enum) =>  a_enum.variants.len(),
        &syn::Data::Union(ref a_union) =>  a_union.fields.named.len()
    };

    count == 0
}
