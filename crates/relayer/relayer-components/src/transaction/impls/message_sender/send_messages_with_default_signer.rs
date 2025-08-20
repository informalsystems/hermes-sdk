use alloc::vec;
use alloc::vec::Vec;

use hermes_chain_components::traits::MessageSenderComponent;
use hermes_prelude::*;

use crate::chain::traits::MessageSender;
use crate::transaction::traits::{CanSendMessagesWithSigner, HasDefaultSigner};

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
        if messages.is_empty() {
            return Ok(vec![]);
        }
        let signer = chain.get_default_signer();
        chain.send_messages_with_signer(signer, &messages).await
    }
}
