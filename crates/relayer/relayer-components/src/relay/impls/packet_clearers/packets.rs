use cgp_core::async_trait;

use crate::chain::traits::queries::packet_acknowledgements::CanQueryPacketAcknowledgements;
use crate::chain::traits::queries::packet_commitments::CanQueryPacketCommitments;
use crate::chain::traits::queries::send_packets::CanQuerySendPackets;
use crate::chain::traits::queries::unreceived_acks_sequences::CanQueryUnreceivedAcksSequences;
use crate::chain::traits::queries::unreceived_packet_sequences::CanQueryUnreceivedPacketSequences;
use crate::chain::types::aliases::{ChannelIdOf, PortIdOf};
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use crate::relay::traits::packet_clearer::PacketClearer;
use crate::relay::traits::packet_relayer::CanRelayPacket;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::task::{CanRunConcurrentTasks, Task};

use super::ack::ClearAckPackets;
use super::receive_packet::ClearReceivePackets;

pub struct ClearAllPackets;

pub struct RelayPacketTask<Relay>
where
    Relay: HasRelayChains,
{
    pub relay: Relay,
    pub packet: Relay::Packet,
}

#[async_trait]
impl<Relay> Task for RelayPacketTask<Relay>
where
    Relay: CanRelayPacket,
{
    async fn run(self) {
        let _ = self.relay.relay_packet(&self.packet).await;
    }
}

#[async_trait]
impl<Relay> PacketClearer<Relay> for ClearAllPackets
where
    Relay: Clone + CanRelayPacket + HasRuntime + CanRaiseRelayChainErrors,
    Relay::DstChain: CanQueryUnreceivedPacketSequences<Relay::SrcChain>
        + CanQueryUnreceivedAcksSequences<Relay::SrcChain>
        + CanQueryPacketAcknowledgements<Relay::SrcChain>,
    Relay::SrcChain:
        CanQueryPacketCommitments<Relay::DstChain> + CanQuerySendPackets<Relay::DstChain>,
    Relay::Runtime: CanRunConcurrentTasks,
{
    async fn clear_packets(
        relay: &Relay,
        src_channel_id: &ChannelIdOf<Relay::SrcChain, Relay::DstChain>,
        src_port_id: &PortIdOf<Relay::SrcChain, Relay::DstChain>,
        dst_channel_id: &ChannelIdOf<Relay::DstChain, Relay::SrcChain>,
        dst_port_id: &PortIdOf<Relay::DstChain, Relay::SrcChain>,
    ) -> Result<(), Relay::Error> {
        ClearReceivePackets::clear_packets(
            relay,
            src_channel_id,
            src_port_id,
            dst_channel_id,
            dst_port_id,
        )
        .await?;
        ClearAckPackets::clear_packets(
            relay,
            src_channel_id,
            src_port_id,
            dst_channel_id,
            dst_port_id,
        )
        .await?;

        Ok(())
    }
}
