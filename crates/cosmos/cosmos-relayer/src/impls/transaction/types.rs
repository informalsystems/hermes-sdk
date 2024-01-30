use cgp_core::Async;
use futures::lock::MutexGuard;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components::transaction::traits::nonce::guard::HasNonceGuard;
use hermes_relayer_components::transaction::traits::types::{
    ProvideFeeType, ProvideNonceType, ProvideSignerType, ProvideTransactionHashType,
    ProvideTransactionType, ProvideTxResponseType,
};
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_proto::cosmos::tx::v1beta1::{Fee, TxRaw};
use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer::chain::cosmos::types::tx::SignedTx;
use ibc_relayer::keyring::Secp256k1KeyPair;
use prost::Message;
use tendermint::Hash as TxHash;
use tendermint_rpc::endpoint::tx::Response as TxResponse;

use crate::contexts::transaction::CosmosTxContext;
use crate::impls::transaction::component::CosmosTxComponents;

impl ProvideRuntime<CosmosTxContext> for CosmosTxComponents {
    fn runtime(chain: &CosmosTxContext) -> &HermesRuntime {
        &chain.runtime
    }
}

impl<Chain> ProvideSignerType<Chain> for CosmosTxComponents
where
    Chain: Async,
{
    type Signer = Secp256k1KeyPair;
}

impl<Chain> ProvideNonceType<Chain> for CosmosTxComponents
where
    Chain: Async,
{
    type Nonce = Account;
}

impl<Chain> ProvideTransactionType<Chain> for CosmosTxComponents
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

impl<Chain> ProvideFeeType<Chain> for CosmosTxComponents
where
    Chain: Async,
{
    type Fee = Fee;
}

impl<Chain> ProvideTransactionHashType<Chain> for CosmosTxComponents
where
    Chain: Async,
{
    type TxHash = TxHash;
}

impl<Chain> ProvideTxResponseType<Chain> for CosmosTxComponents
where
    Chain: Async,
{
    type TxResponse = TxResponse;
}

impl HasNonceGuard for CosmosTxContext {
    type NonceGuard<'a> = (MutexGuard<'a, ()>, Account);

    fn deref_nonce<'a, 'b>((_, nonce): &'a Self::NonceGuard<'b>) -> &'a Account {
        nonce
    }
}
