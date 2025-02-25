use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::types::fee::{
    FeeTypeComponent, ProvideFeeType,
};
use hermes_relayer_components::transaction::traits::types::nonce::NonceTypeProviderComponent;
use hermes_relayer_components::transaction::traits::types::signer::SignerTypeProviderComponent;
use hermes_relayer_components::transaction::traits::types::transaction::{
    ProvideTransactionType, TransactionTypeComponent,
};
use hermes_relayer_components::transaction::traits::types::tx_hash::{
    ProvideTransactionHashType, TransactionHashTypeComponent,
};
use hermes_relayer_components::transaction::traits::types::tx_response::{
    ProvideTxResponseType, TxResponseTypeComponent,
};
use ibc_proto::cosmos::tx::v1beta1::{Fee, TxRaw};
use prost::Message as _;
use tendermint::hash::Hash;
use tendermint_rpc::endpoint::tx::Response as TxResponse;

use crate::types::key_types::secp256k1::Secp256k1KeyPair;
use crate::types::transaction::account::Account;
use crate::types::transaction::signed_tx::SignedTx;

pub struct UseCosmosTransactionTypes;

delegate_components! {
    UseCosmosTransactionTypes {
        SignerTypeProviderComponent: UseType<Secp256k1KeyPair>,
        NonceTypeProviderComponent: UseType<Account>,
    }
}

#[cgp_provider(TransactionTypeComponent)]
impl<Chain> ProvideTransactionType<Chain> for UseCosmosTransactionTypes
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

#[cgp_provider(FeeTypeComponent)]
impl<Chain> ProvideFeeType<Chain> for UseCosmosTransactionTypes
where
    Chain: Async,
{
    type Fee = Fee;
}

#[cgp_provider(TransactionHashTypeComponent)]
impl<Chain> ProvideTransactionHashType<Chain> for UseCosmosTransactionTypes
where
    Chain: Async,
{
    type TxHash = Hash;
}

#[cgp_provider(TxResponseTypeComponent)]
impl<Chain> ProvideTxResponseType<Chain> for UseCosmosTransactionTypes
where
    Chain: Async,
{
    type TxResponse = TxResponse;
}
