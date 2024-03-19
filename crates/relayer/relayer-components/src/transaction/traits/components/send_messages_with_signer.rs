use alloc::vec::Vec;

use cgp_core::prelude::*;

use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::message::HasMessageType;
use crate::transaction::traits::types::signer::HasSignerType;

#[derive_component(MessagesWithSignerSenderComponent, MessagesWithSignerSender<Chain>)]
#[async_trait]
pub trait CanSendMessagesWithSigner:
    HasSignerType + HasMessageType + HasEventType + HasErrorType
{
    async fn send_messages_with_signer(
        &self,
        signer: &Self::Signer,
        messages: &[Self::Message],
    ) -> Result<Vec<Vec<Self::Event>>, Self::Error>;
}
