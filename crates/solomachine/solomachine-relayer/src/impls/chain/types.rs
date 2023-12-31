use cgp_core::{Async, ErrorRaiser, ProvideErrorType};
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloads, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionHandshakePayloads, HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientEvent, HasCreateClientOptions, HasCreateClientPayload,
};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::HasConnectionOpenInitEvent;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayload;
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayload;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayload;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_runtime::types::error::TokioRuntimeError;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

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

impl<Chain, Counterparty> HasClientStateType<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type ClientState = SolomachineClientState;
}

impl<Chain, Counterparty> HasConsensusStateType<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type ConsensusState = SolomachineConsensusState;
}

impl<Chain, Counterparty> HasCreateClientOptions<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type CreateClientPayloadOptions = ();
}

impl<Chain, Counterparty> HasCreateClientPayload<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type CreateClientPayload = SolomachineCreateClientPayload;
}

impl<Chain, Counterparty> HasUpdateClientPayload<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type UpdateClientPayload = SolomachineUpdateClientPayload;
}

impl<Chain, Counterparty> HasInitConnectionOptionsType<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type InitConnectionOptions = ();
}

impl<Chain, Counterparty> HasInitChannelOptionsType<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type InitChannelOptions = ();
}

impl<Chain, Counterparty> HasConnectionHandshakePayloads<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type ConnectionOpenInitPayload = SolomachineConnectionOpenInitPayload;

    type ConnectionOpenTryPayload = SolomachineConnectionOpenTryPayload;

    type ConnectionOpenAckPayload = SolomachineConnectionOpenAckPayload;

    type ConnectionOpenConfirmPayload = SolomachineConnectionOpenConfirmPayload;
}

impl<Chain, Counterparty> HasChannelHandshakePayloads<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type ChannelOpenTryPayload = SolomachineChannelOpenTryPayload;

    type ChannelOpenAckPayload = SolomachineChannelOpenAckPayload;

    type ChannelOpenConfirmPayload = SolomachineChannelOpenConfirmPayload;
}

impl<Chain, Counterparty> HasReceivePacketPayload<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type ReceivePacketPayload = SolomachineReceivePacketPayload;
}

impl<Chain, Counterparty> HasAckPacketPayload<Counterparty> for SolomachineChain<Chain>
where
    Chain: Async,
{
    type AckPacketPayload = SolomachineAckPacketPayload;
}

impl<Chain, Counterparty> HasTimeoutUnorderedPacketPayload<Counterparty> for SolomachineChain<Chain>
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
