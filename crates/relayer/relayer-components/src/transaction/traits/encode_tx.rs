use cgp_core::prelude::*;

use crate::chain::traits::types::message::HasMessageType;
use crate::transaction::traits::types::fee::HasFeeType;
use crate::transaction::traits::types::nonce::HasNonceType;
use crate::transaction::traits::types::signer::HasSignerType;
use crate::transaction::traits::types::transaction::HasTransactionType;

#[derive_component(TxEncoderComponent, TxEncoder<TxContext>)]
#[async_trait]
pub trait CanEncodeTx:
    HasSignerType + HasNonceType + HasFeeType + HasMessageType + HasTransactionType + HasErrorType
{
    async fn encode_tx(
        &self,
        signer: &Self::Signer,
        nonce: &Self::Nonce,
        fee: &Self::Fee,
        messages: &[Self::Message],
    ) -> Result<Self::Transaction, Self::Error>;
}
