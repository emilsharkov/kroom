use proc_macro::TokenStream;

mod injectable;

#[proc_macro_attribute]
pub fn injectable(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    injectable::injectable(attr, item)
}