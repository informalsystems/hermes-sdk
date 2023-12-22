use cgp_core::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::std_prelude::*;

#[derive_component(SendPacketsQuerierComponent, SendPacketsQuerier<Chain>)]
#[async_trait]
pub trait CanQuerySendPackets<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasIbcPacketTypes<Counterparty> + HasErrorType
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

#[derive_component(SendPacketQuerierComponent, SendPacketQuerier<Chain>)]
#[async_trait]
pub trait CanQuerySendPacket<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasIbcPacketTypes<Counterparty> + HasErrorType
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
