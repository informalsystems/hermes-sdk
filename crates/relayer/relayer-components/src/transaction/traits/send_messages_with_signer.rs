use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;

use crate::chain::traits::types::message::HasMessageType;
use crate::transaction::traits::types::signer::HasSignerType;

#[cgp_component {
  provider: MessagesWithSignerSender,
  context: Chain,
}]
#[async_trait]
pub trait CanSendMessagesWithSigner:
    HasSignerType + HasMessageType + HasMessageResponseType + HasErrorType
{
    async fn send_messages_with_signer(
        &self,
        signer: &Self::Signer,
        messages: &[Self::Message],
    ) -> Result<Vec<Self::MessageResponse>, Self::Error>;
}
