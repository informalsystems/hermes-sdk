use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::impls::UseEventsMessageResponse;
use hermes_chain_type_components::traits::{
    EventTypeProviderComponent, HasEventType, HasMessageResponseType,
    MessageResponseEventsGetterComponent, MessageResponseTypeComponent,
    MessageTypeProviderComponent,
};
use hermes_relayer_components::chain::traits::{
    AckPacketPayloadTypeProvider, AckPacketPayloadTypeProviderComponent, ChannelEndTypeComponent,
    ChannelOpenAckPayloadTypeComponent, ChannelOpenConfirmPayloadTypeComponent,
    ChannelOpenTryPayloadTypeComponent, CommitmentPrefixTypeComponent,
    ConnectionOpenAckPayloadTypeComponent, ConnectionOpenConfirmPayloadTypeComponent,
    ConnectionOpenInitEventComponent, ConnectionOpenInitPayloadTypeComponent,
    ConnectionOpenTryPayloadTypeComponent, CreateClientEventComponent,
    CreateClientMessageOptionsTypeComponent, CreateClientPayloadOptionsTypeComponent,
    CreateClientPayloadTypeComponent, EventExtractor, EventExtractorComponent, EventTypeProvider,
    HasClientIdType, HasConnectionIdType, InitChannelOptionsTypeComponent,
    InitConnectionOptionsTypeComponent, MessageTypeProvider, ProvideChannelEndType,
    ProvideChannelOpenAckPayloadType, ProvideChannelOpenConfirmPayloadType,
    ProvideChannelOpenTryPayloadType, ProvideCommitmentPrefixType,
    ProvideConnectionOpenAckPayloadType, ProvideConnectionOpenConfirmPayloadType,
    ProvideConnectionOpenInitEvent, ProvideConnectionOpenInitPayloadType,
    ProvideConnectionOpenTryPayloadType, ProvideCreateClientEvent,
    ProvideCreateClientMessageOptionsType, ProvideCreateClientPayloadOptionsType,
    ProvideCreateClientPayloadType, ProvideInitChannelOptionsType,
    ProvideInitConnectionOptionsType, ProvideReceivePacketPayloadType,
    ProvideTimeoutUnorderedPacketPayloadType, ProvideUpdateClientPayloadType,
    ReceivePacketPayloadTypeComponent, TimeoutUnorderedPacketPayloadTypeComponent,
    UpdateClientPayloadTypeComponent,
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

#[cgp_provider(MessageTypeProviderComponent)]
impl<Chain> MessageTypeProvider<Chain> for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type Message = SolomachineMessage;
}

#[cgp_provider(EventTypeProviderComponent)]
impl<Chain> EventTypeProvider<Chain> for ProvideSolomachineChainTypes
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
