use async_trait::async_trait;
use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::components::message_sender::{
    CanSendMessages, MessageSender,
};
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

use crate::traits::has_tx_context::HasTxContext;

pub struct SendMessagesToTxContext;

#[async_trait]
impl<Chain> MessageSender<Chain> for SendMessagesToTxContext
where
    Chain: HasMessageType + HasEventType + HasErrorType + HasTxContext,
    Chain::TxContext:
        CanSendMessages<Message = Chain::Message, Event = Chain::Event, Error = Chain::Error>,
{
    async fn send_messages(
        chain: &Chain,
        messages: Vec<Chain::Message>,
    ) -> Result<Vec<Vec<Chain::Event>>, Chain::Error> {
        chain.tx_context().send_messages(messages).await
    }
}
