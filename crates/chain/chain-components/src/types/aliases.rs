use hermes_chain_type_components::traits::HasOutgoingPacketType;

use crate::traits::{
    HasChainIdType, HasChannelIdType, HasClientIdType, HasConnectionIdType, HasEventType,
    HasHeightType, HasMessageType, HasPortIdType, HasSequenceType, HasTimeoutType,
    HasWriteAckEvent,
};

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
