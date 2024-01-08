use cgp_core::prelude::*;

use crate::transaction::traits::types::HasTxTypes;

#[derive_component(TxEncoderComponent, TxEncoder<TxContext>)]
#[async_trait]
pub trait CanEncodeTx: HasTxTypes {
    async fn encode_tx(
        &self,
        signer: &Self::Signer,
        nonce: &Self::Nonce,
        fee: &Self::Fee,
        messages: &[Self::Message],
    ) -> Result<Self::Transaction, Self::Error>;
}
