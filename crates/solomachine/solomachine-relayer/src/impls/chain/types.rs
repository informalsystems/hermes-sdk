use cgp_core::{Async, CanRaiseError, ErrorRaiser, ProvideErrorType};
use hermes_relayer_components::chain::traits::types::channel::{
    ProvideChannelHandshakePayloadTypes, ProvideInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoder, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ProvideConnectionHandshakePayloadTypes, ProvideInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::ProvideConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientEvent, ProvideCreateClientOptionsType, ProvideCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::HasConnectionOpenInitEvent;
use hermes_relayer_components::chain::traits::types::packets::ack::ProvideAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::update_client::ProvideUpdateClientPayloadType;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_runtime::types::error::TokioRuntimeError;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_proto::google::protobuf::Any;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
use prost::{DecodeError, Message};

use crate::impls::chain::component::SolomachineChainComponents;
use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;
use crate::types::client_state::SolomachineClientState;
use crate::types::consensus_state::SolomachineConsensusState;
use crate::types::event::{
    SolomachineConnectionInitEvent, SolomachineCreateClientEvent, SolomachineEvent,
};
use crate::types::payloads::channel::{
    SolomachineChannelOpenAckPayload, SolomachineChannelOpenConfirmPayload,
    SolomachineChannelOpenTryPayload,
};
use crate::types::payloads::client::{
    SolomachineCreateClientPayload, SolomachineUpdateClientPayload,
};
use crate::types::payloads::connection::{
    SolomachineConnectionOpenAckPayload, SolomachineConnectionOpenConfirmPayload,
    SolomachineConnectionOpenInitPayload, SolomachineConnectionOpenTryPayload,
};
use crate::types::payloads::packet::{
    SolomachineAckPacketPayload, SolomachineReceivePacketPayload,
    SolomachineTimeoutUnorderedPacketPayload,
};
use ibc_proto::ibc::lightclients::solomachine::v3::ClientState as ProtoClientState;

impl<Chain> ProvideErrorType<SolomachineChain<Chain>> for SolomachineChainComponents
where
    Chain: Solomachine,
{
    type Error = Chain::Error;
}

impl<Chain> ErrorRaiser<SolomachineChain<Chain>, TokioRuntimeError> for SolomachineChainComponents
where
    Chain: Solomachine,
{
    fn raise_error(e: TokioRuntimeError) -> Chain::Error {
        Chain::runtime_error(e)
    }
}

impl<Chain> ProvideRuntime<SolomachineChain<Chain>> for SolomachineChainComponents
where
    Chain: Solomachine,
{
    fn runtime(chain: &SolomachineChain<Chain>) -> &HermesRuntime {
        chain.chain.runtime()
    }
}

impl<Chain, Counterparty> ProvideClientStateType<SolomachineChain<Chain>, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ClientState = SolomachineClientState;
}

impl<Chain, Counterparty> ClientStateDecoder<SolomachineChain<Chain>, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
    Counterparty: CanRaiseError<DecodeError>,
{
    fn decode_client_state_bytes(
        client_state_bytes: &[u8],
    ) -> Result<SolomachineClientState, Counterparty::Error> {
        let any_value = Any::decode(client_state_bytes).map_err(Counterparty::raise_error)?;

        let proto_state = ProtoClientState::decode(any_value.value.as_ref())
            .map_err(Counterparty::raise_error)?;

        // TODO: handle TryInto error
        let client_state = proto_state.try_into().unwrap();

        Ok(client_state)
    }
}

impl<Chain, Counterparty> ProvideConsensusStateType<SolomachineChain<Chain>, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ConsensusState = SolomachineConsensusState;
}

impl<Chain, Counterparty> ProvideCreateClientOptionsType<SolomachineChain<Chain>, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type CreateClientOptions = ();
}

impl<Chain, Counterparty> ProvideCreateClientPayloadType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type CreateClientPayload = SolomachineCreateClientPayload;
}

impl<Chain, Counterparty> ProvideUpdateClientPayloadType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type UpdateClientPayload = SolomachineUpdateClientPayload;
}

impl<Chain, Counterparty> ProvideInitConnectionOptionsType<SolomachineChain<Chain>, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type InitConnectionOptions = ();
}

impl<Chain, Counterparty> ProvideInitChannelOptionsType<SolomachineChain<Chain>, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type InitChannelOptions = ();
}

impl<Chain, Counterparty> ProvideConnectionHandshakePayloadTypes<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ConnectionOpenInitPayload = SolomachineConnectionOpenInitPayload;

    type ConnectionOpenTryPayload = SolomachineConnectionOpenTryPayload;

    type ConnectionOpenAckPayload = SolomachineConnectionOpenAckPayload;

    type ConnectionOpenConfirmPayload = SolomachineConnectionOpenConfirmPayload;
}

impl<Chain, Counterparty> ProvideChannelHandshakePayloadTypes<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ChannelOpenTryPayload = SolomachineChannelOpenTryPayload;

    type ChannelOpenAckPayload = SolomachineChannelOpenAckPayload;

    type ChannelOpenConfirmPayload = SolomachineChannelOpenConfirmPayload;
}

impl<Chain, Counterparty> ProvideReceivePacketPayloadType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ReceivePacketPayload = SolomachineReceivePacketPayload;
}

impl<Chain, Counterparty> ProvideAckPacketPayloadType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type AckPacketPayload = SolomachineAckPacketPayload;
}

impl<Chain, Counterparty> ProvideTimeoutUnorderedPacketPayloadType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type TimeoutUnorderedPacketPayload = SolomachineTimeoutUnorderedPacketPayload;
}

impl<Chain, Counterparty> HasCreateClientEvent<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type CreateClientEvent = SolomachineCreateClientEvent;

    fn try_extract_create_client_event(event: Self::Event) -> Option<Self::CreateClientEvent> {
        match event {
            SolomachineEvent::CreateClient(e) => Some(e),
            _ => None,
        }
    }

    fn create_client_event_client_id(event: &Self::CreateClientEvent) -> &ClientId {
        &event.client_id
    }
}

impl<Chain, Counterparty> HasConnectionOpenInitEvent<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type ConnectionOpenInitEvent = SolomachineConnectionInitEvent;

    fn try_extract_connection_open_init_event(
        event: Self::Event,
    ) -> Option<Self::ConnectionOpenInitEvent> {
        match event {
            SolomachineEvent::ConnectionInit(e) => Some(e),
            _ => None,
        }
    }

    fn connection_open_init_event_connection_id(
        event: &Self::ConnectionOpenInitEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}
