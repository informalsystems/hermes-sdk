use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemTrait};

use crate::helper::component_name::derive_component_name_struct;
use crate::helper::component_spec::ComponentSpec;
use crate::helper::consumer_impl::derive_consumer_impl;
use crate::helper::provider_impl::derive_provider_impl;
use crate::helper::provider_trait::derive_provider_trait;

pub fn derive_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let provider_spec = parse_macro_input!(attr as ComponentSpec);

    let consumer_trait = parse_macro_input!(item as ItemTrait);

    let provider_name = &provider_spec.provider_name;
    let context_type = &provider_spec.context_type;

    let component_struct = derive_component_name_struct(provider_name);

    let provider_trait =
        derive_provider_trait(&consumer_trait, provider_name, context_type).unwrap();

    let consumer_impl = derive_consumer_impl(&consumer_trait, provider_name, context_type);

    let provider_impl = derive_provider_impl(&provider_trait);

    let mut output = consumer_trait.to_token_stream();

    output.extend(component_struct.to_token_stream());
    output.extend(provider_trait.to_token_stream());
    output.extend(consumer_impl.to_token_stream());
    output.extend(provider_impl.to_token_stream());

    output.into()
}
