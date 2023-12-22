use hermes_relayer_components::transaction::traits::logs::nonce::CanLogNonce;
use hermes_relayer_runtime::types::log::value::LogValue;
use ibc_relayer::chain::cosmos::types::account::Account;

use crate::contexts::transaction::CosmosTxContext;

impl CanLogNonce for CosmosTxContext {
    fn log_nonce(nonce: &Account) -> LogValue<'_> {
        LogValue::Debug(nonce)
    }
}
