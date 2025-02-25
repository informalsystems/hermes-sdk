use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;

use crate::transaction::traits::nonce::allocate_nonce::CanAllocateNonce;
use crate::transaction::traits::parse_events::CanParseTxMessageResponse;
use crate::transaction::traits::send_messages_with_signer::{
    MessagesWithSignerSender, MessagesWithSignerSenderComponent,
};
use crate::transaction::traits::send_messages_with_signer_and_nonce::CanSendMessagesWithSignerAndNonce;

pub struct AllocateNonceAndSendMessages;

#[cgp_provider(MessagesWithSignerSenderComponent)]
impl<Chain> MessagesWithSignerSender<Chain> for AllocateNonceAndSendMessages
where
    Chain: HasMessageResponseType
        + CanAllocateNonce
        + CanSendMessagesWithSignerAndNonce
        + CanParseTxMessageResponse,
{
    async fn send_messages_with_signer(
        chain: &Chain,
        signer: &Chain::Signer,
        messages: &[Chain::Message],
    ) -> Result<Vec<Chain::MessageResponse>, Chain::Error> {
        let (_guard, nonce) = chain.allocate_nonce(signer).await?;

        let response = chain
            .send_messages_with_signer_and_nonce(signer, &nonce, messages)
            .await?;

        let responses = Chain::parse_tx_message_response(response)?;

        Ok(responses)
    }
}
