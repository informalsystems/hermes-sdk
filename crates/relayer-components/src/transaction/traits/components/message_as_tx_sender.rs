use cgp_core::prelude::*;

use crate::std_prelude::*;
use crate::transaction::traits::types::HasTxTypes;

#[derive_component(MessageAsTxSenderComponent, MessageAsTxSender<TxContext>)]
#[async_trait]
pub trait CanSendMessagesAsTx: HasTxTypes {
    async fn send_messages_as_tx(
        &self,
        signer: &Self::Signer,
        nonce: &Self::Nonce,
        messages: &[Self::Message],
    ) -> Result<Self::TxResponse, Self::Error>;
}
