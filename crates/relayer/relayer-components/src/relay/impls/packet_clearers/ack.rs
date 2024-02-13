use cgp_core::async_trait;

use crate::chain::traits::queries::packet_acknowledgements::CanQueryPacketAcknowledgements;
use crate::chain::traits::queries::packet_commitments::CanQueryPacketCommitments;
use crate::chain::traits::queries::send_packets::CanQuerySendPackets;
use crate::chain::traits::queries::unreceived_acks_sequences::CanQueryUnreceivedAcksSequences;
use crate::chain::types::aliases::{ChannelIdOf, PortIdOf};
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use crate::relay::traits::packet_clearer::PacketClearer;
use crate::relay::traits::packet_relayer::CanRelayPacket;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::task::{CanRunConcurrentTasks, Task};

pub struct ClearAckPackets;

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
impl<Relay> PacketClearer<Relay> for ClearAckPackets
where
    Relay: Clone + CanRelayPacket + HasRuntime + CanRaiseRelayChainErrors,
    Relay::DstChain: CanQueryUnreceivedAcksSequences<Relay::SrcChain>
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
        let dst_chain = relay.dst_chain();
        let src_chain = relay.src_chain();

        let (commitment_sequences, _) = src_chain
            .query_packet_commitments(src_channel_id, src_port_id)
            .await
            .map_err(Relay::raise_error)?;

        let acks_and_height_on_counterparty = dst_chain
            .query_packet_acknowlegements(dst_channel_id, dst_port_id, &commitment_sequences)
            .await
            .unwrap();

        if let Some((acks_on_counterparty, height)) = acks_and_height_on_counterparty {
            let unreceived_ack_sequences = dst_chain
                .query_unreceived_acknowledgments_sequences(
                    dst_channel_id,
                    dst_port_id,
                    &acks_on_counterparty,
                )
                .await
                .map_err(Relay::raise_error)?;

            let ack_packets = src_chain
                .query_send_packets_from_sequences(
                    src_channel_id,
                    src_port_id,
                    dst_channel_id,
                    dst_port_id,
                    &unreceived_ack_sequences,
                    &height,
                )
                .await
                .map_err(Relay::raise_error)?;

            let tasks = ack_packets
                .into_iter()
                .map(|packet| RelayPacketTask {
                    relay: relay.clone(),
                    packet,
                })
                .collect();

            relay.runtime().run_concurrent_tasks(tasks).await;
        }

        Ok(())
    }
}
