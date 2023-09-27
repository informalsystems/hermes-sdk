use proc_macro2::Span;
use syn::Ident;

pub fn provider_to_component_name(provider_name: &Ident) -> Ident {
    let mut raw_name = provider_name.to_string();
    raw_name.push_str("Component");
    Ident::new(&raw_name, Span::call_site())
}
