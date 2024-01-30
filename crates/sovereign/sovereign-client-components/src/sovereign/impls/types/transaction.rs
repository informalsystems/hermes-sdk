use cgp_core::Async;
use ed25519_dalek::SigningKey;
use hermes_relayer_components::transaction::traits::types::{ProvideNonceType, ProvideSignerType};

pub struct ProvideSovereignTransactionTypes;

impl<Chain> ProvideNonceType<Chain> for ProvideSovereignTransactionTypes
where
    Chain: Async,
{
    type Nonce = u64;
}

impl<Chain> ProvideSignerType<Chain> for ProvideSovereignTransactionTypes
where
    Chain: Async,
{
    type Signer = SigningKey;
}
