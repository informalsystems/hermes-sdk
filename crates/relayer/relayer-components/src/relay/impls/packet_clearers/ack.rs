use alloc::format;
use cgp_core::async_trait;
use core::fmt::Display;

use crate::chain::traits::queries::ack_packets::CanQueryAckPackets;
use crate::chain::traits::queries::packet_acknowledgements::CanQueryPacketAcknowledgements;
use crate::chain::traits::queries::packet_commitments::CanQueryPacketCommitments;
use crate::chain::traits::queries::unreceived_acks_sequences::CanQueryUnreceivedAcksSequences;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::chain::types::aliases::{ChannelIdOf, PortIdOf};
use crate::chain::types::aliases::{HeightOf, WriteAckEventOf};
use crate::logger::traits::log::CanLog;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use crate::relay::traits::packet_clearer::PacketClearer;
use crate::relay::traits::packet_relayers::ack_packet::CanRelayAckPacket;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::task::{CanRunConcurrentTasks, Task};

pub struct ClearAckPackets;

pub struct RelayPacketTask<Relay>
where
    Relay: HasRelayChains,
    Relay::DstChain: HasWriteAckEvent<Relay::SrcChain>,
{
    pub relay: Relay,
    pub height: HeightOf<Relay::DstChain>,
    pub packet: Relay::Packet,
    pub ack: WriteAckEventOf<Relay::DstChain, Relay::SrcChain>,
}

#[async_trait]
impl<Relay> Task for RelayPacketTask<Relay>
where
    Relay: CanRelayAckPacket + CanLog,
    Relay::Packet: Display,
    Relay::Error: Display,
    Relay::DstChain: HasWriteAckEvent<Relay::SrcChain>,
{
    async fn run(self) {
        if let Err(e) = self
            .relay
            .relay_ack_packet(&self.height, &self.packet, &self.ack)
            .await
        {
            self.relay.log_error(&format!(
                "failed to relay packet the packet {} during ack packet clearing: {e}",
                self.packet
            ));
        }
    }
}

#[async_trait]
impl<Relay> PacketClearer<Relay> for ClearAckPackets
where
    Relay: Clone + CanRelayAckPacket + HasRuntime + CanRaiseRelayChainErrors + CanLog,
    Relay::DstChain: CanQueryAckPackets<Relay::SrcChain>
        + HasIbcPacketTypes<Relay::SrcChain, OutgoingPacket = Relay::Packet>
        + CanQueryPacketAcknowledgements<Relay::SrcChain>,
    Relay::SrcChain: CanQueryPacketCommitments<Relay::DstChain>
        + CanQueryUnreceivedAcksSequences<Relay::DstChain>,
    Relay::Runtime: CanRunConcurrentTasks,
    Relay::Packet: Display,
    Relay::Error: Display,
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
            let unreceived_ack_sequences = src_chain
                .query_unreceived_acknowledgments_sequences(
                    src_channel_id,
                    src_port_id,
                    &acks_on_counterparty,
                )
                .await
                .map_err(Relay::raise_error)?;

            let ack_packets = dst_chain
                .query_ack_packets_from_sequences(
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
                .map(|(packet, ack)| RelayPacketTask {
                    height: height.clone(),
                    relay: relay.clone(),
                    packet,
                    ack,
                })
                .collect();

            relay.runtime().run_concurrent_tasks(tasks).await;
        }

        Ok(())
    }
}
