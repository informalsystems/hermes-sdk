use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_components::traits::send_message::MessageSenderComponent;

use crate::chain::traits::send_message::MessageSender;
use crate::transaction::traits::default_signer::HasDefaultSigner;
use crate::transaction::traits::send_messages_with_signer::CanSendMessagesWithSigner;

pub struct SendMessagesWithDefaultSigner;

#[cgp_provider(MessageSenderComponent)]
impl<Chain> MessageSender<Chain> for SendMessagesWithDefaultSigner
where
    Chain: HasDefaultSigner + CanSendMessagesWithSigner,
{
    async fn send_messages(
        chain: &Chain,
        messages: Vec<Chain::Message>,
    ) -> Result<Vec<Chain::MessageResponse>, Chain::Error> {
        let signer = chain.get_default_signer();
        chain.send_messages_with_signer(signer, &messages).await
    }
}
