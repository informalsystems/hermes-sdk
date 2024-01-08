use cgp_core::async_trait;

use crate::chain::traits::components::packet_commitments_querier::CanQueryPacketCommitments;
use crate::chain::traits::components::send_packets_querier::CanQuerySendPackets;
use crate::chain::traits::components::unreceived_packet_sequences_querier::CanQueryUnreceivedPacketSequences;
use crate::chain::types::aliases::{ChannelId, PortId};
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::components::packet_clearer::PacketClearer;
use crate::relay::traits::components::packet_relayer::CanRelayPacket;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::task::{CanRunConcurrentTasks, Task};

pub struct ClearReceivePackets;

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
impl<Relay> PacketClearer<Relay> for ClearReceivePackets
where
    Relay: Clone + CanRelayPacket + HasRuntime,
    Relay::DstChain: CanQueryUnreceivedPacketSequences<Relay::SrcChain>,
    Relay::SrcChain:
        CanQueryPacketCommitments<Relay::DstChain> + CanQuerySendPackets<Relay::DstChain>,
    Relay::Runtime: CanRunConcurrentTasks,
{
    async fn clear_packets(
        relay: &Relay,
        src_channel_id: &ChannelId<Relay::SrcChain, Relay::DstChain>,
        src_port_id: &PortId<Relay::SrcChain, Relay::DstChain>,
        dst_channel_id: &ChannelId<Relay::DstChain, Relay::SrcChain>,
        dst_port_id: &PortId<Relay::DstChain, Relay::SrcChain>,
    ) -> Result<(), Relay::Error> {
        let dst_chain = relay.dst_chain();
        let src_chain = relay.src_chain();

        let (commitment_sequences, height) = src_chain
            .query_packet_commitments(src_channel_id, src_port_id)
            .await
            .map_err(Relay::raise_error)?;

        let unreceived_sequences = dst_chain
            .query_unreceived_packet_sequences(dst_channel_id, dst_port_id, &commitment_sequences)
            .await
            .map_err(Relay::raise_error)?;

        let send_packets = src_chain
            .query_send_packets_from_sequences(
                src_channel_id,
                src_port_id,
                dst_channel_id,
                dst_port_id,
                &unreceived_sequences,
                &height,
            )
            .await
            .map_err(Relay::raise_error)?;

        let tasks = send_packets
            .into_iter()
            .map(|packet| RelayPacketTask {
                relay: relay.clone(),
                packet,
            })
            .collect();

        relay.runtime().run_concurrent_tasks(tasks).await;

        Ok(())
    }
}
