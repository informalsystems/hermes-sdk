use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, For, Impl, Plus};
use syn::{parse_quote, Ident, ImplItem, ItemImpl, ItemTrait, Path, TraitItem, TypeParamBound};

use crate::helper::delegate_fn::derive_delegated_fn_impl;

pub fn derive_provider_impl(
    provider_trait: &ItemTrait,
    component_name: &Ident,
    component_params: &Punctuated<Ident, Comma>,
) -> ItemImpl {
    let provider_name = &provider_trait.ident;

    let component_type = Ident::new("Component", Span::call_site());

    let impl_generics = {
        let mut impl_generics = provider_trait.generics.clone();

        impl_generics
            .params
            .insert(0, parse_quote!(#component_type));

        {
            let delegate_constraint: Punctuated<TypeParamBound, Plus> = parse_quote! {
                cgp_core::traits::DelegateComponent< #component_name < #component_params > >
            };

            let mut provider_generics = provider_trait.generics.clone();
            provider_generics.where_clause = None;

            let provider_constraint: Punctuated<TypeParamBound, Plus> = parse_quote! {
                #provider_name #provider_generics
            };

            if let Some(where_clause) = &mut impl_generics.where_clause {
                where_clause.predicates.push(parse_quote! {
                    #component_type : #delegate_constraint
                });

                where_clause.predicates.push(parse_quote! {
                    #component_type :: Delegate : #provider_constraint
                });
            } else {
                impl_generics.where_clause = Some(parse_quote! {
                    where
                        #component_type : #delegate_constraint,
                        #component_type :: Delegate : #provider_constraint
                });
            }
        }

        impl_generics
    };

    let mut impl_fns: Vec<ImplItem> = Vec::new();

    for trait_item in provider_trait.items.iter() {
        if let TraitItem::Fn(trait_fn) = trait_item {
            let impl_fn =
                derive_delegated_fn_impl(&trait_fn.sig, &parse_quote!(#component_type :: Delegate));

            impl_fns.push(ImplItem::Fn(impl_fn))
        }
    }

    let trait_path: Path = {
        let mut trait_generics = provider_trait.generics.clone();
        trait_generics.where_clause = None;

        parse_quote!( #provider_name #trait_generics )
    };

    ItemImpl {
        attrs: provider_trait.attrs.clone(),
        defaultness: None,
        unsafety: provider_trait.unsafety,
        impl_token: Impl::default(),
        generics: impl_generics,
        trait_: Some((None, trait_path, For::default())),
        self_ty: Box::new(parse_quote!(#component_type)),
        brace_token: Brace::default(),
        items: impl_fns,
    }
}
