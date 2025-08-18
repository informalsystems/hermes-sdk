use alloc::vec;
use alloc::vec::Vec;

use hermes_chain_components::traits::{
    CanBuildPacketFromWriteAck, CanQueryChainHeight, CanQueryPacketIsReceived, CanReadPacketFields,
};
use hermes_prelude::*;

use crate::chain::traits::{CanQueryChainStatus, HasWriteAckEvent};
use crate::relay::traits::{
    BatchPacketsRelayer, BatchPacketsRelayerComponent, CanRelayBatchReceivePackets,
    CanRelayBatchTimeoutUnorderedPackets, HasRelayChains, HasRelayPacketType,
};

#[cgp_new_provider(BatchPacketsRelayerComponent)]
impl<Relay, SrcChain, DstChain> BatchPacketsRelayer<Relay> for PerformFullBatchRelay
where
    Relay: CanRelayBatchReceivePackets
        + CanRelayBatchTimeoutUnorderedPackets
        + HasRelayPacketType
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + CanRaiseAsyncError<SrcChain::Error>
        + CanRaiseAsyncError<DstChain::Error>,
    SrcChain: CanQueryChainStatus + CanReadPacketFields<DstChain>,
    DstChain: CanQueryChainStatus
        + CanQueryChainHeight
        + HasWriteAckEvent<SrcChain>
        + CanBuildPacketFromWriteAck<SrcChain>
        + CanQueryPacketIsReceived<SrcChain>,
{
    async fn relay_packets(
        relay: &Relay,
        packets: Vec<&Relay::Packet>,
    ) -> Result<(), Relay::Error> {
        let src_chain = relay.src_chain();
        let dst_chain = relay.dst_chain();

        let mut timeout_packets = vec![];
        let mut timeout_heights = vec![];
        let mut receive_packets = vec![];

        let destination_status = dst_chain
            .query_chain_status()
            .await
            .map_err(Relay::raise_error)?;

        for packet in packets.iter() {
            let is_packet_received = dst_chain
                .query_packet_is_received(
                    &SrcChain::packet_dst_port_id(packet),
                    &SrcChain::packet_dst_channel_id(packet),
                    &SrcChain::packet_sequence(packet),
                )
                .await
                .map_err(Relay::raise_error)?;

            let destination_height = DstChain::chain_status_height(&destination_status);
            let destination_timestamp = DstChain::chain_status_time(&destination_status);

            let packet_timeout_height = SrcChain::packet_timeout_height(packet);
            let packet_timeout_timestamp = SrcChain::packet_timeout_timestamp(packet);

            let has_packet_timed_out = match (packet_timeout_height, packet_timeout_timestamp) {
                (Some(height), Some(timestamp)) => {
                    destination_height > &height
                        || DstChain::has_timed_out(destination_timestamp, &timestamp)
                }
                (Some(height), None) => destination_height > &height,
                (None, Some(timestamp)) => {
                    DstChain::has_timed_out(destination_timestamp, &timestamp)
                }
                (None, None) => {
                    // TODO: raise error?
                    false
                }
            };

            if !is_packet_received && has_packet_timed_out {
                timeout_packets.push(*packet);
                timeout_heights.push(destination_height);
            } else if !is_packet_received {
                receive_packets.push(*packet);
            }
        }

        let src_chain_status = src_chain
            .query_chain_status()
            .await
            .map_err(Relay::raise_error)?;

        let _m_acks = relay
            .relay_receive_packets(
                Relay::SrcChain::chain_status_height(&src_chain_status),
                receive_packets,
            )
            .await?;

        relay
            .relay_timeout_unordered_packets(timeout_heights, timeout_packets)
            .await?;

        Ok(())
    }
}
