use cgp_core::prelude::*;
use cgp_core::{DelegateComponent, HasErrorType};
use hermes_cosmos_client_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_client_components::types::messages::client::create::CosmosCreateClientMessage;
use hermes_cosmos_relayer::chain::impls::create_client_message::DelegateCosmosCreateClientMessageBuilder;
use hermes_cosmos_relayer::types::error::Error;
use hermes_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayloadType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use ibc_relayer_types::tx_msg::Msg;

use crate::types::chain::SolomachineChain;
use crate::types::payloads::client::SolomachineCreateClientPayload;

pub struct BuildCreateSolomachineClientMessage;

impl<Counterparty> DelegateComponent<SolomachineChain<Counterparty>>
    for DelegateCosmosCreateClientMessageBuilder
{
    type Delegate = BuildCreateSolomachineClientMessage;
}

#[async_trait]
impl<Chain, Counterparty> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateSolomachineClientMessage
where
    Chain: HasMessageType<Message = CosmosMessage> + HasErrorType<Error = Error>,
    Counterparty:
        HasCreateClientPayloadType<Chain, CreateClientPayload = SolomachineCreateClientPayload>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        counterparty_payload: SolomachineCreateClientPayload,
    ) -> Result<CosmosMessage, Error> {
        /*let client_state = encode_client_state(&counterparty_payload.client_state)
        .map_err(CosmosBaseError::encode)?;*/

        let client_state = counterparty_payload.client_state.clone().to_any();

        /*let consensus_state =
        encode_consensus_state(&counterparty_payload.client_state.consensus_state)
            .map_err(CosmosBaseError::encode)?;*/

        let consensus_state = counterparty_payload.client_state.consensus_state.to_any();

        let message = CosmosCreateClientMessage {
            client_state,
            consensus_state,
        };

        Ok(message.to_cosmos_message())
    }
}
