use syn::parse::{Parse, ParseStream};
use syn::token::{Gt, Lt};
use syn::Ident;

pub struct ProviderSpec {
    pub name: Ident,
    pub context: Ident,
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
