use syn::{
    parse_macro_input,
    LitStr,
    parse::{Parse, ParseStream, Result},
};
use quote::quote;
use proc_macro::TokenStream;

struct ParseTarget(LitStr);

impl Parse for ParseTarget {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(ParseTarget(input.parse()?))
    }
}

#[proc_macro]
pub fn expand(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as ParseTarget).0.value();
    let iter = input_str.chars();
    (quote! { [#(#iter),*] }).into()
}
