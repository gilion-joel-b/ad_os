use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use system_table::SystemTableRefined;

#[proc_macro_attribute]
pub fn entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let function_name = &input.sig.ident;
    let block = &input.block;

    let result = quote! {
        fn #function_name(i: ImageHandle, s: SystemTableRefined) -> u64 {
            #block
        }
    };

    TokenStream::from(result)
}
