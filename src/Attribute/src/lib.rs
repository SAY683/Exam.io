use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn command_registration(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    quote!().into()
}
