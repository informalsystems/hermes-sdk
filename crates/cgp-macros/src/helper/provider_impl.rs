use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::{Brace, For, Impl, Plus};
use syn::{
    parse_quote, Ident, ImplItem, ImplItemFn, ItemImpl, ItemTrait, Path, TraitItem, TraitItemFn,
    TypeParamBound, Visibility,
};

use crate::helper::component_name::provider_to_component_name;
use crate::helper::signature_args::signature_to_args;

pub fn derive_provider_impl(provider_trait: &ItemTrait) -> ItemImpl {
    let provider_name = &provider_trait.ident;

    let component_type = Ident::new("Component", Span::call_site());

    let component_name = provider_to_component_name(provider_name);

    let impl_generics = {
        let mut impl_generics = provider_trait.generics.clone();

        impl_generics
            .params
            .insert(0, parse_quote!(#component_type));

        {
            let delegate_constraint: Punctuated<TypeParamBound, Plus> = parse_quote! {
                cgp_core::traits::DelegateComponent< #component_name >
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
            let impl_fn = derive_provider_impl_fn(trait_fn, &component_type);

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

pub fn derive_provider_impl_fn(func: &TraitItemFn, component_type: &Ident) -> ImplItemFn {
    let fn_name = &func.sig.ident;

    let mut fn_generics = func.sig.generics.clone();
    fn_generics.where_clause = None;

    let args = signature_to_args(&func.sig);

    let await_expr: TokenStream = if func.sig.asyncness.is_some() {
        quote!( .await )
    } else {
        TokenStream::new()
    };

    let body = parse_quote!({
        #component_type :: Delegate :: #fn_name #fn_generics (
            #args
        ) #await_expr
    });

    ImplItemFn {
        attrs: func.attrs.clone(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: func.sig.clone(),
        block: body,
    }
}
