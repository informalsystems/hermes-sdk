use cgp::prelude::*;
use hermes_cosmos_chain_components::types::CosmosCreateClientPayload;
use hermes_relayer_components::chain::traits::{
    CreateClientMessageBuilder, CreateClientMessageBuilderComponent,
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType, HasMessageType,
};

use crate::types::message::SolomachineMessage;

pub struct BuildCreateCosmosClientMessage;

#[cgp_provider(CreateClientMessageBuilderComponent)]
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
