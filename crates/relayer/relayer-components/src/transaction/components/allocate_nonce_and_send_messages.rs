use alloc::vec::Vec;

use crate::transaction::traits::components::send_messages_with_signer::MessagesWithSignerSender;
use crate::transaction::traits::components::send_messages_with_signer_and_nonce::CanSendMessagesWithSignerAndNonce;
use crate::transaction::traits::event::CanParseTxResponseAsEvents;
use crate::transaction::traits::nonce::allocate_nonce::CanAllocateNonce;

pub struct AllocateNonceAndSendMessages;

impl<Chain> MessagesWithSignerSender<Chain> for AllocateNonceAndSendMessages
where
    Chain: CanAllocateNonce + CanSendMessagesWithSignerAndNonce + CanParseTxResponseAsEvents,
{
    async fn send_messages_with_signer(
        chain: &Chain,
        signer: &Chain::Signer,
        messages: &[Chain::Message],
    ) -> Result<Vec<Vec<Chain::Event>>, Chain::Error> {
        let nonce = chain.allocate_nonce(signer).await?;

        let response = chain
            .send_messages_with_signer_and_nonce(signer, &nonce, messages)
            .await?;

        let events = Chain::parse_tx_response_as_events(response)?;

        Ok(events)
    }
}
