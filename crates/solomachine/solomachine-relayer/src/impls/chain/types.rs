use core::time::Duration;

use cgp_core::{Async, ErrorRaiser, ProvideErrorType};
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, ProvideEncodingType,
};
use hermes_relayer_components::chain::traits::types::channel::{
    ProvideChannelOpenAckPayloadType, ProvideChannelOpenConfirmPayloadType,
    ProvideChannelOpenTryPayloadType, ProvideInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetter, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ProvideConnectionOpenAckPayloadType, ProvideConnectionOpenConfirmPayloadType,
    ProvideConnectionOpenInitPayloadType, ProvideConnectionOpenTryPayloadType,
    ProvideInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::ProvideConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::{
    ProvideCreateClientEvent, ProvideCreateClientMessageOptionsType,
    ProvideCreateClientPayloadOptionsType, ProvideCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::ProvideConnectionOpenInitEvent;
use hermes_relayer_components::chain::traits::types::packets::ack::ProvideAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::update_client::ProvideUpdateClientPayloadType;
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeGetter;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc_relayer_types::Height;

use crate::context::encoding::SolomachineEncoding;
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

impl<Chain> RuntimeGetter<SolomachineChain<Chain>> for SolomachineChainComponents
where
    Chain: Solomachine,
{
    fn runtime(chain: &SolomachineChain<Chain>) -> &HermesRuntime {
        chain.chain.runtime()
    }
}

impl<Chain> ProvideEncodingType<SolomachineChain<Chain>> for SolomachineChainComponents
where
    Chain: Async,
{
    type Encoding = SolomachineEncoding;
}

impl<Chain> DefaultEncodingGetter<SolomachineChain<Chain>> for SolomachineChainComponents
where
    Chain: Async,
{
    fn default_encoding() -> &'static SolomachineEncoding {
        &SolomachineEncoding
    }
}

impl<Chain, Counterparty> ProvideClientStateType<SolomachineChain<Chain>, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ClientState = SolomachineClientState;
}

// TODO: properly implement solomachine client state fields
impl<Chain, Counterparty> ClientStateFieldsGetter<SolomachineChain<Chain>, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    fn client_state_latest_height(client_state: &SolomachineClientState) -> Height {
        Height::new(0, client_state.sequence).unwrap()
    }

    fn client_state_is_frozen(client_state: &SolomachineClientState) -> bool {
        client_state.is_frozen
    }

    fn client_state_has_expired(
        _client_state: &SolomachineClientState,
        _elapsed: Duration,
    ) -> bool {
        false
    }
}

impl<Chain, Counterparty> ProvideConsensusStateType<SolomachineChain<Chain>, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ConsensusState = SolomachineConsensusState;
}

impl<Chain, Counterparty>
    ProvideCreateClientPayloadOptionsType<SolomachineChain<Chain>, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type CreateClientPayloadOptions = ();
}

impl<Chain, Counterparty> ProvideCreateClientMessageOptionsType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type CreateClientMessageOptions = ();
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

impl<Chain, Counterparty> ProvideConnectionOpenInitPayloadType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ConnectionOpenInitPayload = SolomachineConnectionOpenInitPayload;
}

impl<Chain, Counterparty> ProvideConnectionOpenTryPayloadType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ConnectionOpenTryPayload = SolomachineConnectionOpenTryPayload;
}

impl<Chain, Counterparty> ProvideConnectionOpenAckPayloadType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ConnectionOpenAckPayload = SolomachineConnectionOpenAckPayload;
}

impl<Chain, Counterparty> ProvideConnectionOpenConfirmPayloadType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ConnectionOpenConfirmPayload = SolomachineConnectionOpenConfirmPayload;
}

impl<Chain, Counterparty> ProvideChannelOpenTryPayloadType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ChannelOpenTryPayload = SolomachineChannelOpenTryPayload;
}

impl<Chain, Counterparty> ProvideChannelOpenAckPayloadType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ChannelOpenAckPayload = SolomachineChannelOpenAckPayload;
}

impl<Chain, Counterparty> ProvideChannelOpenConfirmPayloadType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
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

impl<Chain, Counterparty> ProvideCreateClientEvent<SolomachineChain<Chain>, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type CreateClientEvent = SolomachineCreateClientEvent;

    fn try_extract_create_client_event(
        event: SolomachineEvent,
    ) -> Option<SolomachineCreateClientEvent> {
        match event {
            SolomachineEvent::CreateClient(e) => Some(e),
            _ => None,
        }
    }

    fn create_client_event_client_id(event: &SolomachineCreateClientEvent) -> &ClientId {
        &event.client_id
    }
}

impl<Chain, Counterparty> ProvideConnectionOpenInitEvent<SolomachineChain<Chain>, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type ConnectionOpenInitEvent = SolomachineConnectionInitEvent;

    fn try_extract_connection_open_init_event(
        event: SolomachineEvent,
    ) -> Option<SolomachineConnectionInitEvent> {
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
