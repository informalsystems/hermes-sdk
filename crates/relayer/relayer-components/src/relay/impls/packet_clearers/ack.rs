use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::task::{CanRunConcurrentTasks, Task};

use crate::chain::traits::queries::ack_packets::CanQueryAckPackets;
use crate::chain::traits::queries::packet_acknowledgements::CanQueryPacketAcknowledgements;
use crate::chain::traits::queries::packet_commitments::CanQueryPacketCommitments;
use crate::chain::traits::queries::unreceived_acks_sequences::CanQueryUnreceivedAcksSequences;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::chain::types::aliases::{ChannelIdOf, HeightOf, PortIdOf, WriteAckEventOf};
use crate::relay::impls::packet_clearers::receive_packet::{
    ClearPacketAction, LogClearPacketError,
};
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use crate::relay::traits::packet_clearer::PacketClearer;
use crate::relay::traits::packet_relayers::ack_packet::CanRelayAckPacket;

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

impl<Relay> Task for RelayPacketTask<Relay>
where
    Relay: CanRelayAckPacket + HasLogger,
    Relay::DstChain: HasWriteAckEvent<Relay::SrcChain>,
    Relay::Logger: for<'a> CanLog<LogClearPacketError<'a, Relay>>,
{
    async fn run(self) {
        let res = self
            .relay
            .relay_ack_packet(
                &self.height,
                &self.packet,
                Relay::DstChain::write_acknowledgement(&self.ack).as_ref(),
            )
            .await;

        if let Err(e) = res {
            self.relay
                .logger()
                .log(
                    "failed to relay packet during ack packet clearing",
                    &LogClearPacketError {
                        relay: &self.relay,
                        packet: &self.packet,
                        clear_action: ClearPacketAction::ClearReceivePacket,
                        error: &e,
                    },
                )
                .await;
        }
    }
}

impl<Relay> PacketClearer<Relay> for ClearAckPackets
where
    Relay: Clone + HasRuntime + CanRaiseRelayChainErrors + HasLogger,
    Relay::DstChain: CanQueryAckPackets<Relay::SrcChain>
        + HasIbcPacketTypes<Relay::SrcChain, OutgoingPacket = Relay::Packet>
        + CanQueryPacketAcknowledgements<Relay::SrcChain>,
    Relay::SrcChain: CanQueryPacketCommitments<Relay::DstChain>
        + CanQueryUnreceivedAcksSequences<Relay::DstChain>,
    Relay::Runtime: CanRunConcurrentTasks,
    RelayPacketTask<Relay>: Task,
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
            .map_err(Relay::raise_error)?;

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
