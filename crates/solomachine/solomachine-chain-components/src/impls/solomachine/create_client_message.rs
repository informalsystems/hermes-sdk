use cgp::core::error::HasAsyncErrorType;
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientPayload;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

use crate::types::message::SolomachineMessage;

pub struct BuildCreateCosmosClientMessage;

impl<Chain, Counterparty> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateCosmosClientMessage
where
    Chain: HasMessageType<Message = SolomachineMessage>
        + HasCreateClientMessageOptionsType<Counterparty>
        + HasAsyncErrorType,
    Counterparty:
        HasCreateClientPayloadType<Chain, CreateClientPayload = CosmosCreateClientPayload>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        _options: &Chain::CreateClientMessageOptions,
        counterparty_payload: CosmosCreateClientPayload,
    ) -> Result<SolomachineMessage, Chain::Error> {
        let message: SolomachineMessage =
            SolomachineMessage::CosmosCreateClient(Box::new(counterparty_payload));

        Ok(message)
    }
}
