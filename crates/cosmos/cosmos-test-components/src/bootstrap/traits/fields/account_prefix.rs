use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;

#[derive_component(AccountPrefixGetterComponent, AccountPrefixGetter<Bootstrap>)]
pub trait HasAccountPrefix: Async {
    fn account_prefix(&self) -> &str;
}

impl<Bootstrap> AccountPrefixGetter<Bootstrap> for UseContext
where
    Bootstrap: Async + HasField<symbol!("account_prefix"), Value = String>,
{
    fn account_prefix(bootstrap: &Bootstrap) -> &str {
        bootstrap.get_field(PhantomData)
    }
}
