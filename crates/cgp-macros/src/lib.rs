#![no_std]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemTrait};

#[proc_macro_attribute]
pub fn derive_component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);

    input.to_token_stream().into()
}
