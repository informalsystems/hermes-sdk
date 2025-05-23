use hermes_prelude::*;

use crate::chain::traits::HasMessageType;
use crate::transaction::traits::{HasNonceType, HasSignerType, HasTxResponseType};

#[cgp_component {
  provider: MessagesWithSignerAndNonceSender,
  context: TxContext,
}]
#[async_trait]
pub trait CanSendMessagesWithSignerAndNonce:
    HasSignerType + HasNonceType + HasMessageType + HasTxResponseType + HasAsyncErrorType
{
    async fn send_messages_with_signer_and_nonce(
        &self,
        signer: &Self::Signer,
        nonce: &Self::Nonce,
        messages: &[Self::Message],
    ) -> Result<Self::TxResponse, Self::Error>;
}
