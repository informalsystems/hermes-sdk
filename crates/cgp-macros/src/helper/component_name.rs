use proc_macro2::Span;
use syn::{parse_quote, Ident, ItemStruct};

pub fn provider_to_component_name(provider_name: &Ident) -> Ident {
    let mut raw_name = provider_name.to_string();
    raw_name.push_str("Component");
    Ident::new(&raw_name, Span::call_site())
}

pub fn derive_component_name_struct(provider_name: &Ident) -> ItemStruct {
    let component_name = provider_to_component_name(provider_name);

    parse_quote!(pub struct #component_name ;)
}
