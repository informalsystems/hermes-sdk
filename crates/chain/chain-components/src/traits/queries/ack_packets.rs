use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;
use hermes_chain_type_components::traits::types::ibc::sequence::HasSequenceType;

use crate::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::types::aliases::WriteAckEventOf;

#[cgp_component {
  name: AckPacketsQuerierComponent,
  provider: AckPacketsQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryAckPackets<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasWriteAckEvent<Counterparty>
    + HasErrorType
where
    Counterparty: HasOutgoingPacketType<Self>
        + HasChannelIdType<Self>
        + HasPortIdType<Self>
        + HasSequenceType<Self>,
{
    /// Given a list of sequences, a channel and port will query a list of outgoing
    /// ack packets which have not been relayed.
    async fn query_ack_packets_from_sequences(
        &self,
        channel_id: &Counterparty::ChannelId,
        port_id: &Counterparty::PortId,
        counterparty_channel_id: &Self::ChannelId,
        counterparty_port_id: &Self::PortId,
        sequences: &[Counterparty::Sequence],
        // The height is given to query the packets from a specific height.
        // This height should be the same as the query height from the
        // `CanQueryPacketAcknowledgements` made on the same chain.
        height: &Self::Height,
    ) -> Result<
        Vec<(
            Counterparty::OutgoingPacket,
            WriteAckEventOf<Self, Counterparty>,
        )>,
        Self::Error,
    >;
}

#[cgp_component {
  name: AckPacketQuerierComponent,
  provider: AckPacketQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryAckPacket<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasWriteAckEvent<Counterparty>
    + HasErrorType
where
    Counterparty: HasOutgoingPacketType<Self>
        + HasChannelIdType<Self>
        + HasPortIdType<Self>
        + HasSequenceType<Self>,
{
    /// Given a list of sequences, a channel and port will query a list of outgoing
    /// ack packets which have not been relayed.
    async fn query_ack_packet_from_sequence(
        &self,
        channel_id: &Counterparty::ChannelId,
        port_id: &Counterparty::PortId,
        counterparty_channel_id: &Self::ChannelId,
        counterparty_port_id: &Self::PortId,
        sequence: &Counterparty::Sequence,
        height: &Self::Height,
    ) -> Result<
        (
            Counterparty::OutgoingPacket,
            WriteAckEventOf<Self, Counterparty>,
        ),
        Self::Error,
    >;
}
