use cgp_core::HasErrorType;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::client::create::CosmosCreateClientMessage;
use hermes_cosmos_relayer::types::error::Error;
use hermes_protobuf_encoding_components::types::Any;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use ibc_relayer_types::tx_msg::Msg;

use crate::types::payloads::client::SolomachineCreateClientPayload;

pub struct BuildCreateSolomachineClientMessage;

impl<Chain, Counterparty> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateSolomachineClientMessage
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasCreateClientMessageOptionsType<Counterparty>
        + HasErrorType<Error = Error>,
    Counterparty:
        HasCreateClientPayloadType<Chain, CreateClientPayload = SolomachineCreateClientPayload>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        _options: &Chain::CreateClientMessageOptions,
        counterparty_payload: SolomachineCreateClientPayload,
    ) -> Result<CosmosMessage, Error> {
        let client_state = counterparty_payload.client_state.clone().to_any();

        let consensus_state = counterparty_payload.client_state.consensus_state.to_any();

        let message = CosmosCreateClientMessage {
            client_state: Any {
                type_url: client_state.type_url,
                value: client_state.value,
            },
            consensus_state: Any {
                type_url: consensus_state.type_url,
                value: consensus_state.value,
            },
        };

        Ok(message.to_cosmos_message())
    }
}
