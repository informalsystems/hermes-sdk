use cgp_core::prelude::*;

use crate::chain::traits::types::message::HasMessageType;
use crate::transaction::traits::types::nonce::HasNonceType;
use crate::transaction::traits::types::signer::HasSignerType;
use crate::transaction::traits::types::tx_response::HasTxResponseType;

#[derive_component(MessagesWithSignerAndNonceSenderComponent, MessagesWithSignerAndNonceSender<TxContext>)]
#[async_trait]
pub trait CanSendMessagesWithSignerAndNonce:
    HasSignerType + HasNonceType + HasMessageType + HasTxResponseType + HasErrorType
{
    async fn send_messages_with_signer_and_nonce(
        &self,
        signer: &Self::Signer,
        nonce: &Self::Nonce,
        messages: &[Self::Message],
    ) -> Result<Self::TxResponse, Self::Error>;
}
