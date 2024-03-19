use core::ops::Deref;

use futures::lock::MutexGuard;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components::transaction::traits::nonce::guard::HasNonceGuard;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer::chain::cosmos::types::account::Account;

use crate::contexts::transaction::CosmosTxContext;
use crate::impls::transaction::components::CosmosTxComponents;

impl ProvideRuntime<CosmosTxContext> for CosmosTxComponents {
    fn runtime(chain: &CosmosTxContext) -> &HermesRuntime {
        &chain.runtime
    }
}

pub struct NonceGuard<'a> {
    pub mutex_guard: MutexGuard<'a, ()>,
    pub account: Account,
}

impl<'a> Deref for NonceGuard<'a> {
    type Target = Account;

    fn deref(&self) -> &Account {
        &self.account
    }
}

impl HasNonceGuard for CosmosTxContext {
    type NonceGuard<'a> = NonceGuard<'a>;
}
