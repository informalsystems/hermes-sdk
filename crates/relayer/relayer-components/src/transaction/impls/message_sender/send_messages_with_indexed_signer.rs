use alloc::vec::Vec;
use alloc::{format, vec};

use hermes_chain_components::traits::{HasChainId, MessageSenderComponent};
use hermes_logging_components::traits::CanLog;
use hermes_logging_components::types::LevelDebug;
use hermes_prelude::*;

use crate::chain::traits::MessageSender;
use crate::transaction::traits::{CanSendMessagesWithSigner, HasMutexForSigner, HasSigner};

pub struct SendMessagesWithRoundRobinSigner;

#[cgp_provider(MessageSenderComponent)]
impl<Chain> MessageSender<Chain> for SendMessagesWithRoundRobinSigner
where
    Chain:
        HasChainId + HasSigner + HasMutexForSigner + CanSendMessagesWithSigner + CanLog<LevelDebug>,
{
    async fn send_messages(
        chain: &Chain,
        messages: Vec<Chain::Message>,
    ) -> Result<Vec<Chain::MessageResponse>, Chain::Error> {
        if messages.is_empty() {
            return Ok(vec![]);
        }
        let (signer_index_mutex, max_signer_length) = chain.mutex_for_signer();
        let mut signer_index = signer_index_mutex.lock().await;
        let signer = chain.get_signer(*signer_index)?;
        // Only update the signer index if there are additional signers
        if max_signer_length > 0 {
            // Since index 0 is used for the `key_entry` wallet, the maximum value for the index is 1 higher
            // than the number of additional wallets.
            *signer_index = (*signer_index + 1) % max_signer_length;
        }
        chain
            .log(
                &format!(
                    "{} will send message using round robin signer: {signer:?}",
                    chain.chain_id()
                ),
                &LevelDebug,
            )
            .await;
        chain.send_messages_with_signer(signer, &messages).await
    }
}
