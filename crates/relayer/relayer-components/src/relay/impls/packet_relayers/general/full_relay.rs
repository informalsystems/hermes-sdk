use cgp::prelude::*;
use hermes_chain_components::traits::packet::fields::CanReadPacketFields;
use hermes_chain_components::traits::packet::from_write_ack::CanBuildPacketFromWriteAck;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::queries::packet_is_received::CanQueryPacketIsReceived;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;

use crate::chain::traits::queries::chain_status::CanQueryChainStatus;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::relay::traits::chains::{HasRelayChains, HasRelayPacketType, PacketOf};
use crate::relay::traits::packet_relayer::{PacketRelayer, PacketRelayerComponent};
use crate::relay::traits::packet_relayers::ack_packet::CanRelayAckPacket;
use crate::relay::traits::packet_relayers::receive_packet::CanRelayReceivePacket;
use crate::relay::traits::packet_relayers::timeout_unordered_packet::CanRelayTimeoutUnorderedPacket;

pub struct LogRelayPacketAction<'a, Relay>
where
    Relay: HasRelayChains,
{
    pub relay: &'a Relay,
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
        + HasLogger
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + CanRaiseAsyncError<SrcChain::Error>
        + CanRaiseAsyncError<DstChain::Error>,
    SrcChain: CanQueryChainStatus + CanReadPacketFields<DstChain>,
    DstChain: CanQueryChainStatus
        + CanQueryChainHeight
        + HasWriteAckEvent<SrcChain>
        + CanBuildPacketFromWriteAck<SrcChain>
        + CanQueryPacketIsReceived<SrcChain>,
    Relay::Logger: for<'a> CanLog<LogRelayPacketAction<'a, Relay>>,
{
    async fn relay_packet(relay: &Relay, packet: &Relay::Packet) -> Result<(), Relay::Error> {
        let src_chain = relay.src_chain();
        let dst_chain = relay.dst_chain();
        let logger = relay.logger();

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
            logger
                .log(
                    "relaying timeout unordered packet",
                    &LogRelayPacketAction {
                        relay,
                        packet,
                        relay_progress: RelayPacketProgress::RelayTimeoutUnorderedPacket,
                    },
                )
                .await;

            relay
                .relay_timeout_unordered_packet(destination_height, packet)
                .await?;

            logger
                .log(
                    "successfully relayed timeout unordered packet",
                    &LogRelayPacketAction {
                        relay,
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

            logger
                .log(
                    "relaying receive packet",
                    &LogRelayPacketAction {
                        relay,
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

            logger
                .log(
                    "successfully relayed receive packet",
                    &LogRelayPacketAction {
                        relay,
                        packet,
                        relay_progress: RelayPacketProgress::RelayRecvPacket,
                    },
                )
                .await;

            if let Some(ack) = m_ack {
                logger
                    .log(
                        "relaying ack packet using ack event returned from recv-packet event",
                        &LogRelayPacketAction {
                            relay,
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

                logger
                    .log(
                        "successfully relayed ack packet",
                        &LogRelayPacketAction {
                            relay,
                            packet,
                            relay_progress: RelayPacketProgress::RelayAckPacket,
                        },
                    )
                    .await;
            }
        } else {
            logger
                .log(
                    "skip relaying receive packet as it has already been received",
                    &LogRelayPacketAction {
                        relay,
                        packet,
                        relay_progress: RelayPacketProgress::SkipRelayAckPacket,
                    },
                )
                .await;
        }

        Ok(())
    }
}
