use cgp_core::prelude::*;

use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::message::HasMessageType;
use crate::std_prelude::*;
use crate::transaction::traits::components::send_messages_with_signer::CanSendMessagesWithSigner;
use crate::transaction::traits::types::HasSignerType;

#[async_trait]
pub trait CanSendSingleMessageWithSigner:
    HasSignerType + HasMessageType + HasEventType + HasErrorType
{
    async fn send_message_with_signer(
        &self,
        signer: &Self::Signer,
        message: Self::Message,
    ) -> Result<Vec<Self::Event>, Self::Error>;
}

#[async_trait]
impl<Chain> CanSendSingleMessageWithSigner for Chain
where
    Chain: CanSendMessagesWithSigner,
{
    async fn send_message_with_signer(
        &self,
        signer: &Self::Signer,
        message: Chain::Message,
    ) -> Result<Vec<Chain::Event>, Chain::Error> {
        let events = self
            .send_messages_with_signer(signer, &[message])
            .await?
            .into_iter()
            .flatten()
            .collect();

        Ok(events)
    }
}
