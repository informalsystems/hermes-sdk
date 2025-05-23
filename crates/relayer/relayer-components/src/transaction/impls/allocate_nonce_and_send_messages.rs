use alloc::vec::Vec;

use hermes_chain_type_components::traits::HasMessageResponseType;
use hermes_prelude::*;

use crate::transaction::traits::{
    CanAllocateNonce, CanParseTxMessageResponse, CanSendMessagesWithSignerAndNonce,
    MessagesWithSignerSender, MessagesWithSignerSenderComponent,
};

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
