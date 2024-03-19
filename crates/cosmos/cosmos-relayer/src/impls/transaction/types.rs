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

impl HasNonceGuard for CosmosTxContext {
    type NonceGuard<'a> = (MutexGuard<'a, ()>, Account);

    fn deref_nonce<'a, 'b>((_, nonce): &'a Self::NonceGuard<'b>) -> &'a Account {
        nonce
    }
}
