use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::impls::types::message_response::UseEventsMessageResponse;
use hermes_chain_type_components::traits::types::event::HasEventType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_cosmos_chain_components::components::client::{
    MessageResponseEventsGetterComponent, MessageResponseTypeComponent,
};
use hermes_relayer_components::chain::traits::commitment_prefix::ProvideCommitmentPrefixType;
use hermes_relayer_components::chain::traits::extract_data::EventExtractor;
use hermes_relayer_components::chain::traits::types::channel::{
    ProvideChannelEndType, ProvideChannelOpenAckPayloadType, ProvideChannelOpenConfirmPayloadType,
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
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;
use hermes_relayer_components::chain::traits::types::ibc::{HasClientIdType, HasConnectionIdType};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::ProvideConnectionOpenInitEvent;
use hermes_relayer_components::chain::traits::types::message::ProvideMessageType;
use hermes_relayer_components::chain::traits::types::packets::ack::ProvideAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::update_client::ProvideUpdateClientPayloadType;
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

impl<Chain> ProvideMessageType<Chain> for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type Message = SolomachineMessage;
}

impl<Chain> ProvideEventType<Chain> for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type Event = SolomachineEvent;
}

impl<Chain, Counterparty> ProvideChannelEndType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ChannelEnd = ChannelEnd;
}

impl<Chain> ProvideCommitmentPrefixType<Chain> for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type CommitmentPrefix = String;
}

impl<Chain, Counterparty> ProvideCreateClientPayloadOptionsType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type CreateClientPayloadOptions = ();
}

impl<Chain, Counterparty> ProvideCreateClientMessageOptionsType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type CreateClientMessageOptions = ();
}

impl<Chain, Counterparty> ProvideCreateClientPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type CreateClientPayload = SolomachineCreateClientPayload;
}

impl<Chain, Counterparty> ProvideUpdateClientPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type UpdateClientPayload = SolomachineUpdateClientPayload;
}

impl<Chain, Counterparty> ProvideInitConnectionOptionsType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type InitConnectionOptions = ();
}

impl<Chain, Counterparty> ProvideInitChannelOptionsType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type InitChannelOptions = ();
}

impl<Chain, Counterparty> ProvideConnectionOpenInitPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ConnectionOpenInitPayload = SolomachineConnectionOpenInitPayload;
}

impl<Chain, Counterparty> ProvideConnectionOpenTryPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ConnectionOpenTryPayload = SolomachineConnectionOpenTryPayload;
}

impl<Chain, Counterparty> ProvideConnectionOpenAckPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ConnectionOpenAckPayload = SolomachineConnectionOpenAckPayload;
}

impl<Chain, Counterparty> ProvideConnectionOpenConfirmPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ConnectionOpenConfirmPayload = SolomachineConnectionOpenConfirmPayload;
}

impl<Chain, Counterparty> ProvideChannelOpenTryPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ChannelOpenTryPayload = SolomachineChannelOpenTryPayload;
}

impl<Chain, Counterparty> ProvideChannelOpenAckPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ChannelOpenAckPayload = SolomachineChannelOpenAckPayload;
}

impl<Chain, Counterparty> ProvideChannelOpenConfirmPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ChannelOpenConfirmPayload = SolomachineChannelOpenConfirmPayload;
}

impl<Chain, Counterparty> ProvideReceivePacketPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type ReceivePacketPayload = SolomachineReceivePacketPayload;
}

impl<Chain, Counterparty> ProvideAckPacketPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type AckPacketPayload = SolomachineAckPacketPayload;
}

impl<Chain, Counterparty> ProvideTimeoutUnorderedPacketPayloadType<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type TimeoutUnorderedPacketPayload = SolomachineTimeoutUnorderedPacketPayload;
}

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

impl<Chain, Counterparty> ProvideConnectionOpenInitEvent<Chain, Counterparty>
    for ProvideSolomachineChainTypes
where
    Chain: HasMessageResponseType<MessageResponse = Vec<SolomachineEvent>>
        + HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>,
{
    type ConnectionOpenInitEvent = SolomachineConnectionInitEvent;

    fn try_extract_connection_open_init_event(
        events: &Vec<SolomachineEvent>,
    ) -> Option<SolomachineConnectionInitEvent> {
        events.iter().find_map(|event| match event {
            SolomachineEvent::ConnectionInit(e) => Some(e.clone()),
            _ => None,
        })
    }

    fn connection_open_init_event_connection_id(
        event: &Self::ConnectionOpenInitEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}
