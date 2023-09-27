use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use quote::{format_ident, ToTokens};
use syn::parse::Parse;
use syn::{parse_quote, FnArg, Receiver, TraitItemFn};

pub fn iter_parse_and_replace_self_type<I, T>(vals: I, replaced_ident: &Ident) -> syn::Result<I>
where
    I: IntoIterator<Item = T> + FromIterator<T>,
    T: ToTokens + Parse,
{
    vals.into_iter()
        .map(|val| parse_and_replace_self_type(&val, replaced_ident))
        .collect()
}

pub fn parse_and_replace_self_type<T>(val: &T, replaced_ident: &Ident) -> syn::Result<T>
where
    T: ToTokens + Parse,
{
    let stream = replace_self_type(val.to_token_stream(), replaced_ident);
    syn::parse2(stream)
}

pub fn replace_self_type(stream: TokenStream, replaced_ident: &Ident) -> TokenStream {
    let self_type = format_ident!("Self");

    stream
        .into_iter()
        .map(|tree| match tree {
            TokenTree::Ident(ident) => {
                if ident == self_type {
                    TokenTree::Ident(replaced_ident.clone())
                } else {
                    TokenTree::Ident(ident)
                }
            }
            TokenTree::Group(group) => {
                let replaced_stream = replace_self_type(group.stream(), replaced_ident);
                let replace_group = Group::new(group.delimiter(), replaced_stream);

                TokenTree::Group(replace_group)
            }
            TokenTree::Punct(punct) => TokenTree::Punct(punct),
            TokenTree::Literal(lit) => TokenTree::Literal(lit),
        })
        .collect()
}

pub fn replace_self_receiver(func: &mut TraitItemFn, replaced_type: &Ident) {
    let owned_self: Receiver = parse_quote!(self);
    let ref_self: Receiver = parse_quote!(&self);
    let mut_self: Receiver = parse_quote!(&mut self);

    if let Some(arg) = func.sig.inputs.first_mut() {
        if let FnArg::Receiver(receiver) = arg {
            let replaced_var = to_snake_case(replaced_type);

            if receiver == &owned_self {
                *arg = parse_quote!(#replaced_var : #replaced_type);
            } else if receiver == &ref_self {
                *arg = parse_quote!(#replaced_var : & #replaced_type);
            } else if receiver == &mut_self {
                *arg = parse_quote!(#replaced_var : &mut #replaced_type);
            }
        }
    }
}

pub fn to_snake_case(val: &Ident) -> Ident {
    let mut acc = String::new();
    let mut prev = '_';
    for ch in val.to_string().chars() {
        if ch.is_uppercase() && prev != '_' {
            acc.push('_');
        }
        acc.push(ch);
        prev = ch;
    }

    let raw_res = acc.to_lowercase();

    Ident::new(&raw_res, Span::call_site())
}
