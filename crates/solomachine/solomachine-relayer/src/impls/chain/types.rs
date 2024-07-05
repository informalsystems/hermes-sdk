use cgp_core::Async;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, HasEncodingType, ProvideEncodingType,
};
use hermes_relayer_components::chain::traits::types::channel::{
    ProvideChannelOpenAckPayloadType, ProvideChannelOpenConfirmPayloadType,
    ProvideChannelOpenTryPayloadType, ProvideInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ProvideConnectionOpenAckPayloadType, ProvideConnectionOpenConfirmPayloadType,
    ProvideConnectionOpenInitPayloadType, ProvideConnectionOpenTryPayloadType,
    ProvideInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    ProvideCreateClientEvent, ProvideCreateClientMessageOptionsType,
    ProvideCreateClientPayloadOptionsType, ProvideCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::connection::ProvideConnectionOpenInitEvent;
use hermes_relayer_components::chain::traits::types::packets::ack::ProvideAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::update_client::ProvideUpdateClientPayloadType;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

use crate::context::encoding::SolomachineEncoding;
use crate::impls::chain::component::SolomachineChainComponents;
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

impl<Chain> ProvideEncodingType<Chain> for SolomachineChainComponents
where
    Chain: Async,
{
    type Encoding = SolomachineEncoding;
}

impl<Chain> DefaultEncodingGetter<Chain> for SolomachineChainComponents
where
    Chain: HasEncodingType<Encoding = SolomachineEncoding>,
{
    fn default_encoding() -> &'static SolomachineEncoding {
        &SolomachineEncoding
    }
}

impl<Chain, Counterparty> ProvideCreateClientPayloadOptionsType<Chain, Counterparty>
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

impl<Chain, Counterparty> ProvideInitConnectionOptionsType<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: Async,
{
    type InitConnectionOptions = ();
}

impl<Chain, Counterparty> ProvideInitChannelOptionsType<Chain, Counterparty>
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

impl<Chain, Counterparty> ProvideCreateClientEvent<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: HasEventType<Event = SolomachineEvent>
        + HasIbcChainTypes<Counterparty, ClientId = ClientId>,
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

impl<Chain, Counterparty> ProvideConnectionOpenInitEvent<Chain, Counterparty>
    for SolomachineChainComponents
where
    Chain: HasEventType<Event = SolomachineEvent>
        + HasIbcChainTypes<Counterparty, ConnectionId = ConnectionId>,
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
