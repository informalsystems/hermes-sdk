use proc_macro2::{Ident, Span};
use syn::{parse_quote, FnArg, Receiver, TraitItemFn};

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

fn to_snake_case(val: &Ident) -> Ident {
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
