use hermes_chain_components::traits::{
    CanBuildPacketFromWriteAck, CanQueryChainHeight, CanQueryPacketIsReceived, CanReadPacketFields,
};
use hermes_logging_components::traits::CanLog;
use hermes_prelude::*;

use crate::chain::traits::{CanQueryChainStatus, HasWriteAckEvent};
use crate::relay::traits::{
    CanRelayAckPacket, CanRelayReceivePacket, CanRelayTimeoutUnorderedPacket, HasRelayChains,
    HasRelayPacketType, PacketOf, PacketRelayer, PacketRelayerComponent,
};

pub struct LogRelayPacketAction<'a, Relay>
where
    Relay: HasRelayChains,
{
    pub packet: &'a PacketOf<Relay>,
    pub relay_progress: RelayPacketProgress,
}

#[derive(Debug)]
pub enum RelayPacketProgress {
    RelayRecvPacket,
    RelayAckPacket,
    RelayTimeoutUnorderedPacket,
    SkipRelayAckPacket,
}

#[cgp_new_provider(PacketRelayerComponent)]
impl<Relay, SrcChain, DstChain> PacketRelayer<Relay> for PerformFullRelay
where
    Relay: CanRelayAckPacket
        + CanRelayReceivePacket
        + CanRelayTimeoutUnorderedPacket
        + HasRelayPacketType
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + for<'a> CanLog<LogRelayPacketAction<'a, Relay>>
        + CanRaiseAsyncError<SrcChain::Error>
        + CanRaiseAsyncError<DstChain::Error>,
    SrcChain: CanQueryChainStatus + CanReadPacketFields<DstChain>,
    DstChain: CanQueryChainStatus
        + CanQueryChainHeight
        + HasWriteAckEvent<SrcChain>
        + CanBuildPacketFromWriteAck<SrcChain>
        + CanQueryPacketIsReceived<SrcChain>,
{
    async fn relay_packet(relay: &Relay, packet: &Relay::Packet) -> Result<(), Relay::Error> {
        let src_chain = relay.src_chain();
        let dst_chain = relay.dst_chain();

        let is_packet_received = dst_chain
            .query_packet_is_received(
                &SrcChain::packet_dst_port_id(packet),
                &SrcChain::packet_dst_channel_id(packet),
                &SrcChain::packet_sequence(packet),
            )
            .await
            .map_err(Relay::raise_error)?;

        let destination_status = dst_chain
            .query_chain_status()
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
            (None, Some(timestamp)) => DstChain::has_timed_out(destination_timestamp, &timestamp),
            (None, None) => {
                // TODO: raise error?
                false
            }
        };

        if !is_packet_received && has_packet_timed_out {
            relay
                .log(
                    "relaying timeout unordered packet",
                    &LogRelayPacketAction {
                        packet,
                        relay_progress: RelayPacketProgress::RelayTimeoutUnorderedPacket,
                    },
                )
                .await;

            relay
                .relay_timeout_unordered_packet(destination_height, packet)
                .await?;

            relay
                .log(
                    "successfully relayed timeout unordered packet",
                    &LogRelayPacketAction {
                        packet,
                        relay_progress: RelayPacketProgress::RelayTimeoutUnorderedPacket,
                    },
                )
                .await;
        } else if !is_packet_received {
            let src_chain_status = src_chain
                .query_chain_status()
                .await
                .map_err(Relay::raise_error)?;

            relay
                .log(
                    "relaying receive packet",
                    &LogRelayPacketAction {
                        packet,
                        relay_progress: RelayPacketProgress::RelayRecvPacket,
                    },
                )
                .await;

            let m_ack = relay
                .relay_receive_packet(
                    Relay::SrcChain::chain_status_height(&src_chain_status),
                    packet,
                )
                .await?;

            relay
                .log(
                    "successfully relayed receive packet",
                    &LogRelayPacketAction {
                        packet,
                        relay_progress: RelayPacketProgress::RelayRecvPacket,
                    },
                )
                .await;

            if let Some(ack) = m_ack {
                relay
                    .log(
                        "relaying ack packet using ack event returned from recv-packet event",
                        &LogRelayPacketAction {
                            packet,
                            relay_progress: RelayPacketProgress::RelayAckPacket,
                        },
                    )
                    .await;

                relay
                    .relay_ack_packet(
                        &dst_chain
                            .query_chain_height()
                            .await
                            .map_err(Relay::raise_error)?,
                        packet,
                        &ack,
                    )
                    .await?;

                relay
                    .log(
                        "successfully relayed ack packet",
                        &LogRelayPacketAction {
                            packet,
                            relay_progress: RelayPacketProgress::RelayAckPacket,
                        },
                    )
                    .await;
            }
        } else {
            relay
                .log(
                    "skip relaying receive packet as it has already been received",
                    &LogRelayPacketAction {
                        packet,
                        relay_progress: RelayPacketProgress::SkipRelayAckPacket,
                    },
                )
                .await;
        }

        Ok(())
    }
}
