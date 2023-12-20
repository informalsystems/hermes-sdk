use alloc::sync::Arc;

use async_trait::async_trait;
use cosmos_client_components::traits::message::CosmosMessage;
use hermes_relayer_components::chain::traits::components::message_sender::{
    CanSendMessages, MessageSender,
};
use tendermint::abci::Event as AbciEvent;

use crate::contexts::transaction::CosmosTxContext;
use crate::types::error::Error;

pub struct CosmosTxInstances;

// Proof that CosmosTxContext implements [`CanSendMessages`].
#[async_trait]
impl MessageSender<CosmosTxContext> for CosmosTxInstances {
    async fn send_messages(
        tx_context: &CosmosTxContext,
        messages: Vec<Arc<dyn CosmosMessage>>,
    ) -> Result<Vec<Vec<Arc<AbciEvent>>>, Error> {
        tx_context.send_messages(messages).await
    }
}
