use futures::lock::MutexGuard;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components::transaction::traits::nonce::guard::HasNonceGuard;
use hermes_relayer_components::transaction::traits::types::{
    HasNonceType, HasSignerType, HasTxTypes,
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

impl HasSignerType for CosmosTxContext {
    type Signer = Secp256k1KeyPair;
}

impl HasNonceType for CosmosTxContext {
    type Nonce = Account;
}

impl HasNonceGuard for CosmosTxContext {
    type NonceGuard<'a> = (MutexGuard<'a, ()>, Account);

    fn deref_nonce<'a, 'b>((_, nonce): &'a Self::NonceGuard<'b>) -> &'a Account {
        nonce
    }
}

impl HasTxTypes for CosmosTxContext {
    type Transaction = SignedTx;

    type Fee = Fee;

    type TxHash = TxHash;

    type TxResponse = TxResponse;

    fn tx_size(signed_tx: &SignedTx) -> usize {
        let tx_raw = TxRaw {
            body_bytes: signed_tx.body_bytes.clone(),
            auth_info_bytes: signed_tx.auth_info_bytes.clone(),
            signatures: signed_tx.signatures.clone(),
        };

        tx_raw.encoded_len()
    }
}
