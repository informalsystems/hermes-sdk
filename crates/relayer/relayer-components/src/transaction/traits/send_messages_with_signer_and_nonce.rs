use cgp::prelude::*;

use crate::chain::traits::types::message::HasMessageType;
use crate::transaction::traits::types::nonce::HasNonceType;
use crate::transaction::traits::types::signer::HasSignerType;
use crate::transaction::traits::types::tx_response::HasTxResponseType;

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
