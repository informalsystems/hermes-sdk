extern crate proc_macro;

mod replace_self;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::token::{Gt, Lt};
use syn::{parse_macro_input, parse_quote, Ident, ItemTrait, TraitItem};

use crate::replace_self::{
    iter_parse_and_replace_self_type, parse_and_replace_self_type, replace_self_receiver,
};

#[proc_macro_attribute]
pub fn derive_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let provider_spec = parse_macro_input!(attr as ProviderSpec);

    let consumer_trait = parse_macro_input!(item as ItemTrait);

    let provider_trait =
        to_provider_trait(&consumer_trait, &provider_spec.name, &provider_spec.context).unwrap();

    let mut output = consumer_trait.to_token_stream();

    output.extend(provider_trait.to_token_stream());

    println!("macro output: {}", output);

    output.into()
}

struct ProviderSpec {
    name: Ident,
    context: Ident,
}

impl Parse for ProviderSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let _: Lt = input.parse()?;

        let context: Ident = input.parse()?;

        let _: Gt = input.parse()?;

        Ok(ProviderSpec { name, context })
    }
}

fn to_provider_trait(
    consumer_trait: &ItemTrait,
    provider_name: &Ident,
    context_type: &Ident,
) -> syn::Result<ItemTrait> {
    let mut provider_trait = consumer_trait.clone();

    provider_trait.ident = provider_name.clone();

    // Add generic parameter `Context` to the front of generics
    {
        provider_trait
            .generics
            .params
            .insert(0, parse_quote!(#context_type));
    }

    // Turn the supertrait constraints into `Context` constraints in the `where` clause
    {
        let context_constraints =
            iter_parse_and_replace_self_type(provider_trait.supertraits.clone(), context_type)?;

        provider_trait.supertraits.clear();

        if let Some(where_clause) = &mut provider_trait.generics.where_clause {
            let mut predicates =
                iter_parse_and_replace_self_type(where_clause.predicates.clone(), context_type)?;

            predicates.push(parse_quote! {
                #context_type : #context_constraints
            });

            where_clause.predicates = predicates;
        } else {
            provider_trait.generics.where_clause = Some(parse_quote! {
                where #context_type : #context_constraints
            });
        }
    }

    // Replace self type and argument into context type argument
    {
        for item in provider_trait.items.iter_mut() {
            let mut replaced_item = parse_and_replace_self_type(item, context_type)?;

            if let TraitItem::Fn(func) = &mut replaced_item {
                replace_self_receiver(func, context_type);
            }

            *item = replaced_item;
        }
    }

    Ok(provider_trait)
}
