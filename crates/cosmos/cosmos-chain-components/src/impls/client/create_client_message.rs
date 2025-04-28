use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_core::encoding_components::traits::{CanConvert, HasDefaultEncoding};
use hermes_core::encoding_components::types::AsBytes;
use hermes_core::relayer_components::chain::traits::{
    CreateClientMessageBuilder, CreateClientMessageBuilderComponent, HasClientStateType,
    HasConsensusStateType, HasCreateClientMessageOptionsType, HasCreateClientPayloadType,
    HasMessageType,
};
use prost_types::Any;

use crate::traits::{CosmosMessage, ToCosmosMessage};
use crate::types::CosmosCreateClientMessage;

pub struct BuildAnyCreateClientMessage;

#[cgp_provider(CreateClientMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding, Payload> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildAnyCreateClientMessage
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasCreateClientMessageOptionsType<Counterparty>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasCreateClientPayloadType<Chain, CreateClientPayload = Payload>
        + HasClientStateType<Chain>
        + HasConsensusStateType<Chain>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Payload: Async
        + HasField<symbol!("client_state"), Value = Counterparty::ClientState>
        + HasField<symbol!("consensus_state"), Value = Counterparty::ConsensusState>,
    Encoding: Async
        + CanConvert<Counterparty::ClientState, Any>
        + CanConvert<Counterparty::ConsensusState, Any>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        _options: &Chain::CreateClientMessageOptions,
        payload: Payload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let encoding = Counterparty::default_encoding();

        let client_state = encoding
            .convert(payload.get_field(PhantomData::<symbol!("client_state")>))
            .map_err(Chain::raise_error)?;

        let consensus_state = encoding
            .convert(payload.get_field(PhantomData::<symbol!("consensus_state")>))
            .map_err(Chain::raise_error)?;

        let message = CosmosCreateClientMessage {
            client_state,
            consensus_state,
        };

        Ok(message.to_cosmos_message())
    }
}
