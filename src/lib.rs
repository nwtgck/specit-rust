extern crate proc_macro;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn it(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
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
    // If not async function
    if fn_asyncness.is_none() {
        // Add #[test] attribute
        // NOTE: A test function can not have #[test] normally, but in some cases such as #[tokio::test] allows to use async function.
        let a: syn::Attribute = syn::parse_quote!{#[test]};
        fn_attrs.push(a);
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
