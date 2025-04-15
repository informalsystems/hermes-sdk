use cgp::prelude::*;

use crate::chain::traits::HasMessageType;
use crate::transaction::traits::{HasFeeType, HasNonceType, HasSignerType, HasTransactionType};

#[cgp_component {
  provider: TxEncoder,
  context: TxContext,
}]
#[async_trait]
pub trait CanEncodeTx:
    HasSignerType + HasNonceType + HasFeeType + HasMessageType + HasTransactionType + HasAsyncErrorType
{
    async fn encode_tx(
        &self,
        signer: &Self::Signer,
        nonce: &Self::Nonce,
        fee: &Self::Fee,
        messages: &[Self::Message],
    ) -> Result<Self::Transaction, Self::Error>;
}
