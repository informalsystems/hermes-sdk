use cgp_core::Async;
use hermes_relayer_components::transaction::traits::nonce::guard::ProvideNonceGuard;
use hermes_relayer_components::transaction::traits::types::fee::ProvideFeeType;
use hermes_relayer_components::transaction::traits::types::nonce::{
    HasNonceType, ProvideNonceType,
};
use hermes_relayer_components::transaction::traits::types::signer::ProvideSignerType;
use hermes_relayer_components::transaction::traits::types::transaction::ProvideTransactionType;
use hermes_relayer_components::transaction::traits::types::tx_hash::ProvideTransactionHashType;
use hermes_relayer_components::transaction::traits::types::tx_response::ProvideTxResponseType;
use ibc_proto::cosmos::tx::v1beta1::{Fee, TxRaw};
use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer::chain::cosmos::types::tx::SignedTx;
use ibc_relayer::keyring::Secp256k1KeyPair;
use prost::Message as _;
use tendermint::hash::Hash;
use tendermint_rpc::endpoint::tx::Response as TxResponse;

use crate::types::nonce_guard::NonceGuard;

pub struct ProvideCosmosTransactionTypes;

impl<Chain> ProvideSignerType<Chain> for ProvideCosmosTransactionTypes
where
    Chain: Async,
{
    type Signer = Secp256k1KeyPair;
}

impl<Chain> ProvideNonceType<Chain> for ProvideCosmosTransactionTypes
where
    Chain: Async,
{
    type Nonce = Account;
}

impl<Chain> ProvideTransactionType<Chain> for ProvideCosmosTransactionTypes
where
    Chain: Async,
{
    type Transaction = SignedTx;

    fn tx_size(signed_tx: &SignedTx) -> usize {
        let tx_raw = TxRaw {
            body_bytes: signed_tx.body_bytes.clone(),
            auth_info_bytes: signed_tx.auth_info_bytes.clone(),
            signatures: signed_tx.signatures.clone(),
        };

        tx_raw.encoded_len()
    }
}

impl<Chain> ProvideFeeType<Chain> for ProvideCosmosTransactionTypes
where
    Chain: Async,
{
    type Fee = Fee;
}

impl<Chain> ProvideTransactionHashType<Chain> for ProvideCosmosTransactionTypes
where
    Chain: Async,
{
    type TxHash = Hash;
}

impl<Chain> ProvideTxResponseType<Chain> for ProvideCosmosTransactionTypes
where
    Chain: Async,
{
    type TxResponse = TxResponse;
}

impl<Chain> ProvideNonceGuard<Chain> for ProvideCosmosTransactionTypes
where
    Chain: HasNonceType<Nonce = Account>,
{
    type NonceGuard<'a> = NonceGuard<'a>;
}
