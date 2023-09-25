extern crate proc_macro;

mod replace_self;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Gt, Lt, Where};
use syn::{
    parse_macro_input, GenericParam, Ident, ItemTrait, Path, PathSegment, PredicateType, TraitItem,
    Type, TypePath, WhereClause, WherePredicate,
};

use crate::replace_self::replace_self;

#[proc_macro_attribute]
pub fn derive_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let provider_spec = parse_macro_input!(attr as ProviderSpec);

    let consumer_trait = parse_macro_input!(item as ItemTrait);

    let provider_trait = to_provider_trait(&consumer_trait, &provider_spec);

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

fn to_provider_trait(consumer_trait: &ItemTrait, provider_spec: &ProviderSpec) -> ItemTrait {
    let mut provider_trait = consumer_trait.clone();

    provider_trait.ident = provider_spec.name.clone();

    // Add generic parameter `Context` to the front of generics
    {
        provider_trait
            .generics
            .params
            .insert(0, GenericParam::Type(provider_spec.context.clone().into()));
    }

    // Turn the supertrait constraints into `Context` constraints in the `where` clause
    {
        let context_constraints = provider_trait.supertraits.clone();

        provider_trait.supertraits.clear();

        let context_path = Path::from(PathSegment {
            ident: provider_spec.context.clone(),
            arguments: syn::PathArguments::None,
        });

        let context_type = Type::from(TypePath {
            qself: None,
            path: context_path,
        });

        let context_predicate = WherePredicate::from(PredicateType {
            lifetimes: None,
            bounded_ty: context_type,
            colon_token: Colon::default(),
            bounds: context_constraints,
        });

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
            let original_stream = item.to_token_stream();
            let replaced_stream = replace_self(original_stream, &provider_spec.context);

            let replaced_item: TraitItem = syn::parse2(replaced_stream).unwrap();
            *item = replaced_item;
        }
    }

    provider_trait
}
