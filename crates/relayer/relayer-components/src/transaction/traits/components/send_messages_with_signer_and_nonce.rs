use cgp_core::prelude::*;

use crate::transaction::traits::types::HasTxTypes;

#[derive_component(MessagesWithSignerAndNonceSenderComponent, MessagesWithSignerAndNonceSender<TxContext>)]
#[async_trait]
pub trait CanSendMessagesWithSignerAndNonce: HasTxTypes {
    async fn send_messages_with_signer_and_nonce(
        &self,
        signer: &Self::Signer,
        nonce: &Self::Nonce,
        messages: &[Self::Message],
    ) -> Result<Self::TxResponse, Self::Error>;
}
