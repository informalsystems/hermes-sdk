use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::counterparty::CanUseCounterparty;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;
use hermes_chain_type_components::traits::types::ibc::sequence::HasSequenceType;

use crate::types::aliases::{ChannelIdOf, PortIdOf};

#[cgp_component {
  provider: SendPacketsQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQuerySendPackets<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasSequenceType<Counterparty>
    + HasOutgoingPacketType<Counterparty>
    + HasErrorType
    + CanUseCounterparty<Counterparty, Counterparty: HasChannelIdType<Self> + HasPortIdType<Self>>
{
    /// Given a list of sequences, a channel and port will query a list of outgoing
    /// packets which have not been relayed.
    async fn query_send_packets_from_sequences(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        counterparty_channel_id: &ChannelIdOf<Counterparty, Self>,
        counterparty_port_id: &PortIdOf<Counterparty, Self>,
        sequences: &[Self::Sequence],
        // The height is given to query the packets from a specific height.
        // This height should be the same as the query height from the
        // `CanQueryPacketCommitments` made on the same chain.
        height: &Self::Height,
    ) -> Result<Vec<Self::OutgoingPacket>, Self::Error>;
}

#[cgp_component {
  provider: SendPacketQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQuerySendPacket<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasSequenceType<Counterparty>
    + HasOutgoingPacketType<Counterparty>
    + HasErrorType
    + CanUseCounterparty<Counterparty, Counterparty: HasChannelIdType<Self> + HasPortIdType<Self>>
{
    /// Given a list of sequences, a channel and port will query a list of outgoing
    /// packets which have not been relayed.
    async fn query_send_packet_from_sequence(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        counterparty_channel_id: &ChannelIdOf<Counterparty, Self>,
        counterparty_port_id: &PortIdOf<Counterparty, Self>,
        sequence: &Self::Sequence,
        height: &Self::Height,
    ) -> Result<Self::OutgoingPacket, Self::Error>;
}
