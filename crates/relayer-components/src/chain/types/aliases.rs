use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAcknowledgementEvent;
use crate::chain::traits::types::message::HasMessageType;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::chain::traits::types::timestamp::HasTimestampType;

pub type IncomingPacket<Chain, Counterparty> =
    <Chain as HasIbcPacketTypes<Counterparty>>::IncomingPacket;

pub type OutgoingPacket<Chain, Counterparty> =
    <Chain as HasIbcPacketTypes<Counterparty>>::OutgoingPacket;

pub type ClientId<Chain, Counterparty> = <Chain as HasIbcChainTypes<Counterparty>>::ClientId;

pub type ConnectionId<Chain, Counterparty> =
    <Chain as HasIbcChainTypes<Counterparty>>::ConnectionId;

pub type ChannelId<Chain, Counterparty> = <Chain as HasIbcChainTypes<Counterparty>>::ChannelId;

pub type PortId<Chain, Counterparty> = <Chain as HasIbcChainTypes<Counterparty>>::PortId;

pub type Sequence<Chain, Counterparty> = <Chain as HasIbcChainTypes<Counterparty>>::Sequence;

pub type ChainId<Chain> = <Chain as HasChainIdType>::ChainId;

pub type Message<Chain> = <Chain as HasMessageType>::Message;

pub type Event<Chain> = <Chain as HasEventType>::Event;

pub type Height<Chain> = <Chain as HasHeightType>::Height;

pub type Timestamp<Chain> = <Chain as HasTimestampType>::Timestamp;

pub type WriteAcknowledgementEvent<Chain, Counterparty> =
    <Chain as HasWriteAcknowledgementEvent<Counterparty>>::WriteAcknowledgementEvent;
