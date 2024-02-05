use cgp_core::prelude::*;

#[derive_component(AccountPrefixGetterComponent, AccountPrefixGetter<Bootstrap>)]
pub trait HasAccountPrefix: Async {
    fn account_prefix(&self) -> &str;
}
