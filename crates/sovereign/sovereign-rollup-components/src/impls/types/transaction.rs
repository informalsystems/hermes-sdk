use cgp_core::Async;
use ed25519_dalek::SigningKey;
use hermes_relayer_components::transaction::traits::nonce::nonce_guard::ProvideNonceGuard;
use hermes_relayer_components::transaction::traits::types::fee::ProvideFeeType;
use hermes_relayer_components::transaction::traits::types::nonce::{
    HasNonceType, ProvideNonceType,
};
use hermes_relayer_components::transaction::traits::types::signer::ProvideSignerType;
use hermes_relayer_components::transaction::traits::types::transaction::ProvideTransactionType;
use hermes_relayer_components::transaction::traits::types::tx_hash::ProvideTransactionHashType;
use hermes_relayer_components::transaction::traits::types::tx_response::ProvideTxResponseType;

use crate::types::tx::nonce_guard::SovereignNonceGuard;
use crate::types::tx::tx_hash::TxHash;
use crate::types::tx::tx_response::TxResponse;

pub struct ProvideSovereignTransactionTypes;

impl<Chain> ProvideTransactionType<Chain> for ProvideSovereignTransactionTypes
where
    Chain: Async,
{
    type Transaction = Vec<u8>;

    fn tx_size(tx: &Vec<u8>) -> usize {
        tx.len()
    }
}

impl<Chain> ProvideNonceType<Chain> for ProvideSovereignTransactionTypes
where
    Chain: Async,
{
    type Nonce = u64;
}

impl<Chain> ProvideFeeType<Chain> for ProvideSovereignTransactionTypes
where
    Chain: Async,
{
    type Fee = u64;
}

impl<Chain> ProvideSignerType<Chain> for ProvideSovereignTransactionTypes
where
    Chain: Async,
{
    type Signer = SigningKey;
}

impl<Chain> ProvideTransactionHashType<Chain> for ProvideSovereignTransactionTypes
where
    Chain: Async,
{
    type TxHash = TxHash;
}

impl<Chain> ProvideTxResponseType<Chain> for ProvideSovereignTransactionTypes
where
    Chain: Async,
{
    type TxResponse = TxResponse;
}

impl<Chain> ProvideNonceGuard<Chain> for ProvideSovereignTransactionTypes
where
    Chain: HasNonceType<Nonce = u64>,
{
    type NonceGuard<'a> = SovereignNonceGuard<'a>;
}
