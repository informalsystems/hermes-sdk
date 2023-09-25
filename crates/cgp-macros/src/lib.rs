#![no_std]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::token::{Gt, Lt};
use syn::{parse_macro_input, Ident, ItemTrait};

#[proc_macro_attribute]
pub fn derive_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let provider = parse_macro_input!(attr as ProviderTrait);

    let body = parse_macro_input!(item as ItemTrait);

    body.to_token_stream().into()
}

struct ProviderTrait {
    name: Ident,
    context: Ident,
}

impl Parse for ProviderTrait {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let _: Lt = input.parse()?;

        let context: Ident = input.parse()?;

        let _: Gt = input.parse()?;

        Ok(ProviderTrait { name, context })
    }
}
