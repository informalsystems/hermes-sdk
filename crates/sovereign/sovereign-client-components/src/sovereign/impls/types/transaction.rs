use cgp_core::Async;
use ed25519_dalek::SigningKey;
use hermes_relayer_components::transaction::traits::types::{
    ProvideFeeType, ProvideNonceType, ProvideSignerType, ProvideTransactionType,
};

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
