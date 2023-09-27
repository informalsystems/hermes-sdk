use syn::punctuated::Punctuated;
use syn::token::{Brace, For, Impl, Plus};
use syn::{
    parse_quote, Ident, ImplItemFn, ItemImpl, ItemTrait, Path, Signature, TraitItemFn,
    TypeParamBound, WherePredicate,
};

use crate::helper::component_name::provider_to_component_name;

pub fn derive_consumer_impl(
    consumer_trait: &ItemTrait,
    provider_name: &Ident,
    context_type: &Ident,
) -> syn::Result<ItemImpl> {
    let consumer_name = &consumer_trait.ident;

    let impl_generics = {
        let mut impl_generics = consumer_trait.generics.clone();

        impl_generics.params.insert(0, parse_quote!(#context_type));

        let mut provider_generics = impl_generics.clone();
        provider_generics.where_clause = None;

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

        impl_generics
    };

    let mut trait_generics = consumer_trait.generics.clone();
    trait_generics.where_clause = None;

    let trait_path: Path = parse_quote!( #consumer_name #trait_generics );

    let impl_block = ItemImpl {
        attrs: consumer_trait.attrs.clone(),
        defaultness: None,
        unsafety: consumer_trait.unsafety.clone(),
        impl_token: Impl::default(),
        generics: impl_generics,
        trait_: Some((None, trait_path, For::default())),
        self_ty: Box::new(parse_quote!(#context_type)),
        brace_token: Brace::default(),
        items: Vec::new(),
    };

    Ok(impl_block)
}

pub fn derive_consumer_impl_fn(
    func: &TraitItemFn,
    consumer_name: &Ident,
    context_type: &Ident,
) -> ImplItemFn {
    todo!()
}

pub fn signature_to_method_call(
    sig: Signature,
    consumer_name: &Ident,
    context_type: &Ident,
) -> ImplItemFn {
    todo!()
}
