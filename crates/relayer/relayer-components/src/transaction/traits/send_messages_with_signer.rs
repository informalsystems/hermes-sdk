use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::HasMessageResponseType;

use crate::chain::traits::HasMessageType;
use crate::transaction::traits::HasSignerType;

#[cgp_component {
  provider: MessagesWithSignerSender,
  context: Chain,
}]
#[async_trait]
pub trait CanSendMessagesWithSigner:
    HasSignerType + HasMessageType + HasMessageResponseType + HasAsyncErrorType
{
    async fn send_messages_with_signer(
        &self,
        signer: &Self::Signer,
        messages: &[Self::Message],
    ) -> Result<Vec<Self::MessageResponse>, Self::Error>;
}
