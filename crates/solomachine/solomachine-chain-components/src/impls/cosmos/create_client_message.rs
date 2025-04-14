use cgp::prelude::*;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::client::create::CosmosCreateClientMessage;
use hermes_encoding_components::traits::{CanConvert, HasDefaultEncoding, HasEncodedType};
use hermes_encoding_components::types::AsBytes;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_relayer_components::chain::traits::{
    CreateClientMessageBuilder, CreateClientMessageBuilderComponent,
    CreateClientMessageOptionsTypeComponent, HasCreateClientMessageOptionsType,
    HasCreateClientPayloadType, HasMessageType, ProvideCreateClientMessageOptionsType,
};

use crate::types::client_state::SolomachineClientState;
use crate::types::consensus_state::SolomachineConsensusState;
use crate::types::payloads::client::SolomachineCreateClientPayload;

pub struct BuildCreateSolomachineClientMessage;

#[cgp_provider(CreateClientMessageOptionsTypeComponent)]
impl<Chain, Counterparty> ProvideCreateClientMessageOptionsType<Chain, Counterparty>
    for BuildCreateSolomachineClientMessage
where
    Chain: Async,
{
    type CreateClientMessageOptions = ();
}

#[cgp_provider(CreateClientMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateSolomachineClientMessage
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasCreateClientMessageOptionsType<Counterparty>
        + HasAsyncErrorType
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasCreateClientPayloadType<Chain, CreateClientPayload = SolomachineCreateClientPayload>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanConvert<SolomachineClientState, Any>
        + CanConvert<SolomachineConsensusState, Any>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        _options: &Chain::CreateClientMessageOptions,
        counterparty_payload: SolomachineCreateClientPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let encoding = Counterparty::default_encoding();

        let client_state = encoding
            .convert(&counterparty_payload.client_state)
            .map_err(Chain::raise_error)?;

        let consensus_state = encoding
            .convert(&counterparty_payload.client_state.consensus_state)
            .map_err(Chain::raise_error)?;

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
