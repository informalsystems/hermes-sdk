use cgp_core::async_trait;

use crate::chain::traits::components::message_sender::MessageSender;
use crate::std_prelude::*;
use crate::transaction::traits::components::send_messages_with_signer::CanSendMessagesWithSigner;
use crate::transaction::traits::signer::HasDefaultSigner;
use crate::transaction::traits::types::HasTxTypes;

pub struct SendMessagesWithDefaultSigner;

#[async_trait]
impl<Chain> MessageSender<Chain> for SendMessagesWithDefaultSigner
where
    Chain: HasTxTypes + HasDefaultSigner + CanSendMessagesWithSigner,
{
    async fn send_messages(
        chain: &Chain,
        messages: Vec<Chain::Message>,
    ) -> Result<Vec<Vec<Chain::Event>>, Chain::Error> {
        let signer = chain.get_default_signer();
        chain.send_messages_with_signer(signer, &messages).await
    }
}
