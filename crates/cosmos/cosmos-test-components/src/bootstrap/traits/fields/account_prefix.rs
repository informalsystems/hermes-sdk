use hermes_prelude::*;

#[cgp_getter {
    provider: AccountPrefixGetter,
}]
pub trait HasAccountPrefix: Async {
    fn account_prefix(&self) -> &str;
}
