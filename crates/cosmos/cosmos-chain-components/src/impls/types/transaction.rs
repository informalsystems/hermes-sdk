use hermes_core::relayer_components::transaction::traits::{
    FeeTypeProviderComponent, NonceTypeProviderComponent, ProvideTransactionType,
    SignerTypeProviderComponent, TransactionTypeComponent, TxHashTypeProviderComponent,
    TxResponseTypeProviderComponent,
};
use hermes_prelude::*;
use ibc_proto::cosmos::tx::v1beta1::{Fee, TxRaw};
use prost::Message as _;
use tendermint::hash::Hash;
use tendermint_rpc::endpoint::tx::Response as TxResponse;

use crate::types::{Account, Secp256k1KeyPair, SignedTx};

pub struct UseCosmosTransactionTypes;

delegate_components! {
    UseCosmosTransactionTypes {
        SignerTypeProviderComponent:
            UseType<Secp256k1KeyPair>,
        NonceTypeProviderComponent:
            UseType<Account>,
        TxResponseTypeProviderComponent:
            UseType<TxResponse>,
        FeeTypeProviderComponent:
            UseType<Fee>,
        TxHashTypeProviderComponent:
            UseType<Hash>,
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
