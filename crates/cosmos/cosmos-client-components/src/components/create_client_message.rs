use alloc::sync::Arc;

use async_trait::async_trait;
use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayload;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::messages::client::create::CosmosCreateClientMessage;
use crate::types::payloads::client::CosmosCreateClientPayload;

pub struct BuildCosmosCreateClientMessage;

#[async_trait]
impl<Chain, Counterparty> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCosmosCreateClientMessage
where
    Chain: HasMessageType<Message = Arc<dyn CosmosMessage>> + HasErrorType,
    Counterparty: HasCreateClientPayload<Chain, CreateClientPayload = CosmosCreateClientPayload>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        payload: Counterparty::CreateClientPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        let message = CosmosCreateClientMessage {
            client_state: payload.client_state.into(),
            consensus_state: payload.consensus_state.into(),
        };

        Ok(message.to_cosmos_message())
    }
}
