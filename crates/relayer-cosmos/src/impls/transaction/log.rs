use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer_components::transaction::traits::logs::nonce::CanLogNonce;
use ibc_relayer_runtime::types::log::value::LogValue;

use crate::contexts::transaction::CosmosTxContext;

impl CanLogNonce for CosmosTxContext {
    fn log_nonce(nonce: &Account) -> LogValue<'_> {
        LogValue::Debug(nonce)
    }
}
