extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Gt, Lt, Where};
use syn::{
    parse_macro_input, GenericParam, Ident, ItemTrait, Path, PathSegment, PredicateType, Type,
    TypePath, WhereClause, WherePredicate,
};

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

    provider_trait
        .generics
        .params
        .push(GenericParam::Type(provider_spec.context.clone().into()));

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

    // let where_clause = provider_trait.generics.where_clause.take();

    // if let Some(where_clause) = where_clause {

    // }

    provider_trait
}
