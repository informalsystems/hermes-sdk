use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemTrait};

use crate::helper::provider_spec::ProviderSpec;
use crate::helper::provider_trait::derive_provider_trait;

pub fn derive_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let provider_spec = parse_macro_input!(attr as ProviderSpec);

    let consumer_trait = parse_macro_input!(item as ItemTrait);

    let provider_trait =
        derive_provider_trait(&consumer_trait, &provider_spec.name, &provider_spec.context)
            .unwrap();

    let mut output = consumer_trait.to_token_stream();

    output.extend(provider_trait.to_token_stream());

    println!("macro output: {}", output);

    output.into()
}
