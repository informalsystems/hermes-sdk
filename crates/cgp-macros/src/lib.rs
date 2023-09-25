#![no_std]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn derive_component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
