use cgp::prelude::*;
use hermes_chain_components::traits::send_message::EmptyMessageResponse;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;

use crate::chain::traits::types::message::HasMessageType;
use crate::transaction::traits::send_messages_with_signer::CanSendMessagesWithSigner;
use crate::transaction::traits::types::signer::HasSignerType;

#[async_trait]
pub trait CanSendSingleMessageWithSigner:
    HasSignerType + HasMessageType + HasMessageResponseType + HasAsyncErrorType
{
    async fn send_message_with_signer(
        &self,
        signer: &Self::Signer,
        message: Self::Message,
    ) -> Result<Self::MessageResponse, Self::Error>;
}

impl<Chain> CanSendSingleMessageWithSigner for Chain
where
    Chain: CanSendMessagesWithSigner + CanRaiseAsyncError<EmptyMessageResponse>,
{
    async fn send_message_with_signer(
        &self,
        signer: &Self::Signer,
        message: Chain::Message,
    ) -> Result<Chain::MessageResponse, Chain::Error> {
        let events = self
            .send_messages_with_signer(signer, &[message])
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Chain::raise_error(EmptyMessageResponse))?;

        Ok(events)
    }
}
