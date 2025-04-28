use cgp::prelude::*;
use hermes_chain_components::traits::EmptyMessageResponse;
use hermes_chain_type_components::traits::HasMessageResponseType;

use crate::chain::traits::HasMessageType;
use crate::transaction::traits::{CanSendMessagesWithSigner, HasSignerType};

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
