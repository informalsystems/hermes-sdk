use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::impls::types::message_response::UseEventsMessageResponse;
use hermes_chain_type_components::traits::fields::message_response_events::MessageResponseEventsGetterComponent;
use hermes_chain_type_components::traits::types::event::{EventTypeComponent, HasEventType};
use hermes_chain_type_components::traits::types::message::MessageTypeComponent;
use hermes_chain_type_components::traits::types::message_response::{
    HasMessageResponseType, MessageResponseTypeComponent,
};
use hermes_relayer_components::chain::traits::commitment_prefix::{
    CommitmentPrefixTypeComponent, ProvideCommitmentPrefixType,
};
use hermes_relayer_components::chain::traits::extract_data::{
    EventExtractor, EventExtractorComponent,
};
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelEndTypeComponent, ChannelOpenAckPayloadTypeComponent,
    ChannelOpenConfirmPayloadTypeComponent, ChannelOpenTryPayloadTypeComponent,
    InitChannelOptionsTypeComponent, ProvideChannelEndType, ProvideChannelOpenAckPayloadType,
    ProvideChannelOpenConfirmPayloadType, ProvideChannelOpenTryPayloadType,
    ProvideInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionOpenAckPayloadTypeComponent, ConnectionOpenConfirmPayloadTypeComponent,
    ConnectionOpenInitPayloadTypeComponent, ConnectionOpenTryPayloadTypeComponent,
    InitConnectionOptionsTypeComponent, ProvideConnectionOpenAckPayloadType,
    ProvideConnectionOpenConfirmPayloadType, ProvideConnectionOpenInitPayloadType,
    ProvideConnectionOpenTryPayloadType, ProvideInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientEventComponent, CreateClientMessageOptionsTypeComponent,
    CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
    ProvideCreateClientEvent, ProvideCreateClientMessageOptionsType,
    ProvideCreateClientPayloadOptionsType, ProvideCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;
use hermes_relayer_components::chain::traits::types::ibc::{HasClientIdType, HasConnectionIdType};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::{
    ConnectionOpenInitEventComponent, ProvideConnectionOpenInitEvent,
};
use hermes_relayer_components::chain::traits::types::message::ProvideMessageType;
use hermes_relayer_components::chain::traits::types::packets::ack::{
    AckPacketPayloadTypeProvider, AckPacketPayloadTypeProviderComponent,
};
use hermes_relayer_components::chain::traits::types::packets::receive::{
    ProvideReceivePacketPayloadType, ReceivePacketPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::packets::timeout::{
    ProvideTimeoutUnorderedPacketPayloadType, TimeoutUnorderedPacketPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::update_client::{
    ProvideUpdateClientPayloadType, UpdateClientPayloadTypeComponent,
};
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::host::types::identifiers::{ClientId, ConnectionId};

use crate::types::event::{
    SolomachineConnectionInitEvent, SolomachineCreateClientEvent, SolomachineEvent,
};
use crate::types::message::SolomachineMessage;
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

pub struct ProvideSolomachineChainTypes;

delegate_components! {
    ProvideSolomachineChainTypes {
        [
            MessageResponseTypeComponent,
            MessageResponseEventsGetterComponent,
        ]:
            UseEventsMessageResponse,
    }
}

#[cgp_provider(MessageTypeComponent)]
impl<Chain> ProvideMessageType<Chain> for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type Message = SolomachineMessage;
}

#[cgp_provider(EventTypeComponent)]
impl<Chain> ProvideEventType<Chain> for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type Event = SolomachineEvent;
}

#[cgp_provider(ChannelEndTypeComponent)]
impl<Chain, Counterparty> ProvideChannelEndType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ChannelEnd = ChannelEnd;
}

#[cgp_provider(CommitmentPrefixTypeComponent)]
impl<Chain> ProvideCommitmentPrefixType<Chain> for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type CommitmentPrefix = String;
}

#[cgp_provider(CreateClientPayloadOptionsTypeComponent)]
impl<Chain, Counterparty> ProvideCreateClientPayloadOptionsType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type CreateClientPayloadOptions = ();
}

#[cgp_provider(CreateClientMessageOptionsTypeComponent)]
impl<Chain, Counterparty> ProvideCreateClientMessageOptionsType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type CreateClientMessageOptions = ();
}

#[cgp_provider(CreateClientPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideCreateClientPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type CreateClientPayload = SolomachineCreateClientPayload;
}

#[cgp_provider(UpdateClientPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideUpdateClientPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type UpdateClientPayload = SolomachineUpdateClientPayload;
}

#[cgp_provider(InitConnectionOptionsTypeComponent)]
impl<Chain, Counterparty> ProvideInitConnectionOptionsType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type InitConnectionOptions = ();
}

#[cgp_provider(InitChannelOptionsTypeComponent)]
impl<Chain, Counterparty> ProvideInitChannelOptionsType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type InitChannelOptions = ();
}

#[cgp_provider(ConnectionOpenInitPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenInitPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ConnectionOpenInitPayload = SolomachineConnectionOpenInitPayload;
}

#[cgp_provider(ConnectionOpenTryPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenTryPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ConnectionOpenTryPayload = SolomachineConnectionOpenTryPayload;
}

#[cgp_provider(ConnectionOpenAckPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenAckPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ConnectionOpenAckPayload = SolomachineConnectionOpenAckPayload;
}

#[cgp_provider(ConnectionOpenConfirmPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenConfirmPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ConnectionOpenConfirmPayload = SolomachineConnectionOpenConfirmPayload;
}

#[cgp_provider(ChannelOpenTryPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideChannelOpenTryPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ChannelOpenTryPayload = SolomachineChannelOpenTryPayload;
}

#[cgp_provider(ChannelOpenAckPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideChannelOpenAckPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ChannelOpenAckPayload = SolomachineChannelOpenAckPayload;
}

#[cgp_provider(ChannelOpenConfirmPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideChannelOpenConfirmPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ChannelOpenConfirmPayload = SolomachineChannelOpenConfirmPayload;
}

#[cgp_provider(ReceivePacketPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideReceivePacketPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ReceivePacketPayload = SolomachineReceivePacketPayload;
}

#[cgp_provider(AckPacketPayloadTypeProviderComponent)]
impl<Chain, Counterparty> AckPacketPayloadTypeProvider<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type AckPacketPayload = SolomachineAckPacketPayload;
}

#[cgp_provider(TimeoutUnorderedPacketPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideTimeoutUnorderedPacketPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type TimeoutUnorderedPacketPayload = SolomachineTimeoutUnorderedPacketPayload;
}

#[cgp_provider(CreateClientEventComponent)]
impl<Chain, Counterparty> ProvideCreateClientEvent<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: HasMessageResponseType<MessageResponse = Vec<SolomachineEvent>>
        + HasClientIdType<Counterparty, ClientId = ClientId>,
{
    type CreateClientEvent = SolomachineCreateClientEvent;

    fn create_client_event_client_id(event: &SolomachineCreateClientEvent) -> &ClientId {
        &event.client_id
    }
}

#[cgp_provider(EventExtractorComponent)]
impl<Chain> EventExtractor<Chain, SolomachineCreateClientEvent> for ProvideSolomachineChainTypes
where
    Chain: HasEventType<Event = SolomachineEvent>,
{
    fn try_extract_from_event(
        _chain: &Chain,
        _tag: PhantomData<SolomachineCreateClientEvent>,
        event: &SolomachineEvent,
    ) -> Option<SolomachineCreateClientEvent> {
        match event {
            SolomachineEvent::CreateClient(e) => Some(e.clone()),
            _ => None,
        }
    }
}

#[cgp_provider(ConnectionOpenInitEventComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenInitEvent<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: HasMessageResponseType<MessageResponse = Vec<SolomachineEvent>>
        + HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>,
{
    type ConnectionOpenInitEvent = SolomachineConnectionInitEvent;

    fn connection_open_init_event_connection_id(
        event: &Self::ConnectionOpenInitEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}

#[cgp_provider(EventExtractorComponent)]
impl<Chain> EventExtractor<Chain, SolomachineConnectionInitEvent> for ProvideSolomachineChainTypes
where
    Chain: HasEventType<Event = SolomachineEvent>,
{
    fn try_extract_from_event(
        _chain: &Chain,
        _tag: PhantomData<SolomachineConnectionInitEvent>,
        event: &SolomachineEvent,
    ) -> Option<SolomachineConnectionInitEvent> {
        match event {
            SolomachineEvent::ConnectionInit(e) => Some(e.clone()),
            _ => None,
        }
    }
}
