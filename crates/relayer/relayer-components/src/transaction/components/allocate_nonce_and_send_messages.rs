use alloc::vec::Vec;
use cgp_core::async_trait;

use crate::transaction::traits::components::nonce_allocater::CanAllocateNonce;
use crate::transaction::traits::components::send_messages_with_signer::MessagesWithSignerSender;
use crate::transaction::traits::components::send_messages_with_signer_and_nonce::CanSendMessagesWithSignerAndNonce;
use crate::transaction::traits::event::CanParseTxResponseAsEvents;
use crate::transaction::traits::types::{HasSignerType, HasTxTypes};

pub struct AllocateNonceAndSendMessages;

#[async_trait]
impl<Chain> MessagesWithSignerSender<Chain> for AllocateNonceAndSendMessages
where
    Chain: HasTxTypes
        + HasSignerType
        + CanAllocateNonce
        + CanSendMessagesWithSignerAndNonce
        + CanParseTxResponseAsEvents,
{
    async fn send_messages_with_signer(
        chain: &Chain,
        signer: &Chain::Signer,
        messages: &[Chain::Message],
    ) -> Result<Vec<Vec<Chain::Event>>, Chain::Error> {
        let nonce = chain.allocate_nonce(signer).await?;

        let response = chain
            .send_messages_with_signer_and_nonce(signer, Chain::deref_nonce(&nonce), messages)
            .await?;

        let events = Chain::parse_tx_response_as_events(response)?;

        Ok(events)
    }
}
