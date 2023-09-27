use syn::punctuated::Punctuated;
use syn::token::{Brace, For, Impl, Plus};
use syn::{parse_quote, Ident, ImplItem, ItemImpl, ItemTrait, Path, TraitItem, TypeParamBound};

use crate::helper::delegate_fn::derive_delegated_fn_impl;

pub fn derive_consumer_impl(
    consumer_trait: &ItemTrait,
    provider_name: &Ident,
    context_type: &Ident,
) -> ItemImpl {
    let consumer_name = &consumer_trait.ident;

    let impl_generics = {
        let mut impl_generics = consumer_trait.generics.clone();

        impl_generics.params.insert(0, parse_quote!(#context_type));

        let mut provider_generics = impl_generics.clone();
        provider_generics.where_clause = None;

        {
            let supertrait_constraints = consumer_trait.supertraits.clone();

            if !supertrait_constraints.is_empty() {
                if let Some(where_clause) = &mut impl_generics.where_clause {
                    where_clause.predicates.push(parse_quote! {
                        #context_type : #supertrait_constraints
                    });
                } else {
                    impl_generics.where_clause = Some(parse_quote! {
                        where #context_type : #supertrait_constraints
                    });
                }
            }
        }

        {
            let has_component_constraint: Punctuated<TypeParamBound, Plus> = parse_quote! {
                cgp_core::traits::HasComponents
            };

            let provider_constraint: Punctuated<TypeParamBound, Plus> = parse_quote! {
                #provider_name #provider_generics
            };

            if let Some(where_clause) = &mut impl_generics.where_clause {
                where_clause.predicates.push(parse_quote! {
                    #context_type : #has_component_constraint
                });

                where_clause.predicates.push(parse_quote! {
                    #context_type :: Components : #provider_constraint
                });
            } else {
                impl_generics.where_clause = Some(parse_quote! {
                    where
                        #context_type : #has_component_constraint,
                        #context_type :: Components : #provider_constraint
                });
            }
        }

        impl_generics
    };

    let mut impl_fns: Vec<ImplItem> = Vec::new();

    for trait_item in consumer_trait.items.iter() {
        if let TraitItem::Fn(trait_fn) = trait_item {
            let impl_fn =
                derive_delegated_fn_impl(&trait_fn.sig, &parse_quote!(#context_type :: Components));

            impl_fns.push(ImplItem::Fn(impl_fn))
        }
    }

    let trait_path: Path = {
        let mut trait_generics = consumer_trait.generics.clone();
        trait_generics.where_clause = None;

        parse_quote!( #consumer_name #trait_generics )
    };

    ItemImpl {
        attrs: consumer_trait.attrs.clone(),
        defaultness: None,
        unsafety: consumer_trait.unsafety,
        impl_token: Impl::default(),
        generics: impl_generics,
        trait_: Some((None, trait_path, For::default())),
        self_ty: Box::new(parse_quote!(#context_type)),
        brace_token: Brace::default(),
        items: impl_fns,
    }
}
