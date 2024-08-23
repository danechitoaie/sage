use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn my_custom_attribute(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        struct H {}
    })
}
