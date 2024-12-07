use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;

use crate::traits::types::ibc::HasIbcChainTypes;

#[cgp_component {
  name: SendPacketsQuerierComponent,
  provider: SendPacketsQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQuerySendPackets<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasOutgoingPacketType<Counterparty> + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    /// Given a list of sequences, a channel and port will query a list of outgoing
    /// packets which have not been relayed.
    async fn query_send_packets_from_sequences(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_port_id: &Counterparty::PortId,
        sequences: &[Self::Sequence],
        // The height is given to query the packets from a specific height.
        // This height should be the same as the query height from the
        // `CanQueryPacketCommitments` made on the same chain.
        height: &Self::Height,
    ) -> Result<Vec<Self::OutgoingPacket>, Self::Error>;
}

#[cgp_component {
  name: SendPacketQuerierComponent,
  provider: SendPacketQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQuerySendPacket<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasOutgoingPacketType<Counterparty> + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    /// Given a list of sequences, a channel and port will query a list of outgoing
    /// packets which have not been relayed.
    async fn query_send_packet_from_sequence(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_port_id: &Counterparty::PortId,
        sequence: &Self::Sequence,
        height: &Self::Height,
    ) -> Result<Self::OutgoingPacket, Self::Error>;
}
