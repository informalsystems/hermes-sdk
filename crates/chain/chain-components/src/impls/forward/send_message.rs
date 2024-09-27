use alloc::vec::Vec;

use cgp::core::error::CanRaiseError;
use cgp::core::inner::HasInner;

use crate::traits::send_message::{CanSendMessages, MessageSender};
use crate::traits::types::event::HasEventType;
use crate::traits::types::message::HasMessageType;

pub struct ForwardSendMessage;

impl<Chain, InChain> MessageSender<Chain> for ForwardSendMessage
where
    Chain:
        HasMessageType + HasEventType + HasInner<Inner = InChain> + CanRaiseError<InChain::Error>,
    InChain: CanSendMessages<Message = Chain::Message, Event = Chain::Event>,
{
    async fn send_messages(
        chain: &Chain,
        messages: Vec<Chain::Message>,
    ) -> Result<Vec<Vec<Chain::Event>>, Chain::Error> {
        chain
            .inner()
            .send_messages(messages)
            .await
            .map_err(Chain::raise_error)
    }
}
