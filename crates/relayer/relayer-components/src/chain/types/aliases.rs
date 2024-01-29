use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::traits::types::message::HasMessageType;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::chain::traits::types::timestamp::HasTimestampType;

pub type IncomingPacketOf<Chain, Counterparty> =
    <Chain as HasIbcPacketTypes<Counterparty>>::IncomingPacket;

pub type OutgoingPacketOf<Chain, Counterparty> =
    <Chain as HasIbcPacketTypes<Counterparty>>::OutgoingPacket;

pub type ClientIdOf<Chain, Counterparty> = <Chain as HasIbcChainTypes<Counterparty>>::ClientId;

pub type ConnectionIdOf<Chain, Counterparty> =
    <Chain as HasIbcChainTypes<Counterparty>>::ConnectionId;

pub type ChannelIdOf<Chain, Counterparty> = <Chain as HasIbcChainTypes<Counterparty>>::ChannelId;

pub type PortIdOf<Chain, Counterparty> = <Chain as HasIbcChainTypes<Counterparty>>::PortId;

pub type SequenceOf<Chain, Counterparty> = <Chain as HasIbcChainTypes<Counterparty>>::Sequence;

pub type ChainIdOf<Chain> = <Chain as HasChainIdType>::ChainId;

pub type MessageOf<Chain> = <Chain as HasMessageType>::Message;

pub type EventOf<Chain> = <Chain as HasEventType>::Event;

pub type HeightOf<Chain> = <Chain as HasHeightType>::Height;

pub type TimestampOf<Chain> = <Chain as HasTimestampType>::Timestamp;

pub type WriteAckEventOf<Chain, Counterparty> =
    <Chain as HasWriteAckEvent<Counterparty>>::WriteAckEvent;
