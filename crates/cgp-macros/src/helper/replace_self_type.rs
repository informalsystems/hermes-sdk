use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::{format_ident, ToTokens};
use syn::parse::Parse;

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
