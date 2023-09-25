use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};

pub fn replace_self(stream: TokenStream, replaced_ident: &Ident) -> TokenStream {
    let self_type = Ident::new("Self", Span::mixed_site());

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
                let replaced_stream = replace_self(group.stream(), replaced_ident);
                let replace_group = Group::new(group.delimiter(), replaced_stream);

                TokenTree::Group(replace_group)
            }
            TokenTree::Punct(punct) => TokenTree::Punct(punct),
            TokenTree::Literal(lit) => TokenTree::Literal(lit),
        })
        .collect()
}
