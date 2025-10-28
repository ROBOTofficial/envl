use proc_macro2::TokenStream;
use quote::quote;

pub fn struct_derive() -> TokenStream {
    quote! {
        #[derive(Debug, Clone, PartialEq)]
    }
}
