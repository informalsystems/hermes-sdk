use alloc::vec::Vec;

use cgp::core::error::CanRaiseError;
use cgp::core::inner::HasInner;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;

use crate::traits::send_message::{CanSendMessages, MessageSender};
use crate::traits::types::message::HasMessageType;

pub struct ForwardSendMessage;

impl<Chain, InChain> MessageSender<Chain> for ForwardSendMessage
where
    Chain: HasMessageType
        + HasMessageResponseType
        + HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>,
    InChain: CanSendMessages<Message = Chain::Message, MessageResponse = Chain::MessageResponse>,
{
    async fn send_messages(
        chain: &Chain,
        messages: Vec<Chain::Message>,
    ) -> Result<Vec<Chain::MessageResponse>, Chain::Error> {
        chain
            .inner()
            .send_messages(messages)
            .await
            .map_err(Chain::raise_error)
    }
}
