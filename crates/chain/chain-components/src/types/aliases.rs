use hermes_chain_type_components::traits::types::ibc::packet::{
    HasIncomingPacketType, HasOutgoingPacketType,
};

use crate::traits::types::chain_id::HasChainIdType;
use crate::traits::types::event::HasEventType;
use crate::traits::types::height::HasHeightType;
use crate::traits::types::ibc::{
    HasChannelIdType, HasClientIdType, HasConnectionIdType, HasPortIdType, HasSequenceType,
};
use crate::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::traits::types::message::HasMessageType;
use crate::traits::types::timestamp::HasTimeoutType;

pub type IncomingPacketOf<Chain, Counterparty> =
    <Chain as HasIncomingPacketType<Counterparty>>::IncomingPacket;

pub type OutgoingPacketOf<Chain, Counterparty> =
    <Chain as HasOutgoingPacketType<Counterparty>>::OutgoingPacket;

pub type ClientIdOf<Chain, Counterparty> = <Chain as HasClientIdType<Counterparty>>::ClientId;

pub type ConnectionIdOf<Chain, Counterparty> =
    <Chain as HasConnectionIdType<Counterparty>>::ConnectionId;

pub type ChannelIdOf<Chain, Counterparty> = <Chain as HasChannelIdType<Counterparty>>::ChannelId;

pub type PortIdOf<Chain, Counterparty> = <Chain as HasPortIdType<Counterparty>>::PortId;

pub type SequenceOf<Chain, Counterparty> = <Chain as HasSequenceType<Counterparty>>::Sequence;

pub type ChainIdOf<Chain> = <Chain as HasChainIdType>::ChainId;

pub type MessageOf<Chain> = <Chain as HasMessageType>::Message;

pub type EventOf<Chain> = <Chain as HasEventType>::Event;

pub type HeightOf<Chain> = <Chain as HasHeightType>::Height;

pub type TimestampOf<Chain> = <Chain as HasTimeoutType>::Timeout;

pub type WriteAckEventOf<Chain, Counterparty> =
    <Chain as HasWriteAckEvent<Counterparty>>::WriteAckEvent;
