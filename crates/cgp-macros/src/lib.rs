extern crate proc_macro;

mod replace_self;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Gt, Lt, Where};
use syn::{
    parse_macro_input, parse_quote, GenericParam, Ident, ItemTrait, WhereClause, WherePredicate,
};

use crate::replace_self::parse_and_replace_self_type;

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
            .insert(0, GenericParam::Type(context_type.clone().into()));
    }

    // Turn the supertrait constraints into `Context` constraints in the `where` clause
    {
        let context_constraints = provider_trait.supertraits.clone();

        provider_trait.supertraits.clear();

        let context_predicate: WherePredicate = parse_quote! {
            #context_type : #context_constraints
        };

        if let Some(where_clause) = &mut provider_trait.generics.where_clause {
            where_clause
                .predicates
                .extend(core::iter::once(context_predicate))
        } else {
            let mut predicates = Punctuated::new();
            predicates.push(context_predicate);

            provider_trait.generics.where_clause = Some(WhereClause {
                where_token: Where::default(),
                predicates,
            });
        }
    }

    // Replace self type and argument into context type argument
    {
        for item in provider_trait.items.iter_mut() {
            let replaced_item = parse_and_replace_self_type(item, &context_type)?;

            *item = replaced_item;
        }
    }

    Ok(provider_trait)
}
