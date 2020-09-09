extern crate proc_macro;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn it(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    general_it(args, input, None)
}

#[cfg(feature = "tokio")]
#[proc_macro_attribute]
pub fn tokio_it(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    general_it(args, input, Some(syn::parse_quote! {#[tokio::test]}))
}

// NOTE: This function is used in macros
#[allow(dead_code)]
fn general_it(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
    async_attribute_option: Option<syn::Attribute>,
) -> proc_macro::TokenStream {
    let lit_str = parse_macro_input!(args as syn::LitStr);
    let input_item = parse_macro_input!(input as syn::Item);
    let syn_fn = match input_item {
        syn::Item::Fn(x) => x,
        _ => panic!("not function"),
    };
    let fn_ret_type = syn_fn.sig.output;
    let fn_block = syn_fn.block;
    let mut fn_attrs = syn_fn.attrs;
    let fn_asyncness = syn_fn.sig.asyncness;

    // If async function
    if fn_asyncness.is_some() {
        // If async attribute is found
        if let Some(async_attribute) = async_attribute_option {
            fn_attrs.push(async_attribute);
        }
    } else {
        fn_attrs.push(syn::parse_quote! {#[test]});
    }

    let ident = {
        let s = lit_str.value();
        let new_str: String = s
            .chars()
            .map(|x| match x {
                'A'..='Z' | 'a'..='z' | '0'..='9' => x,
                _ => '_',
            })
            .collect();
        syn::Ident::new(&new_str, syn_fn.sig.ident.span())
    };

    let q = quote! {
        #[allow(non_snake_case)]
        #(#fn_attrs)*
        #fn_asyncness fn #ident() #fn_ret_type #fn_block
    };
    q.into()
}
