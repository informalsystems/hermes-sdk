use cgp::prelude::*;
use hermes_chain_components::traits::packet::fields::CanReadPacketFields;
use hermes_chain_components::traits::packet::from_write_ack::CanBuildPacketFromWriteAck;
use hermes_chain_components::traits::queries::packet_is_cleared::CanQueryPacketIsCleared;
use hermes_chain_components::traits::queries::packet_is_received::CanQueryPacketIsReceived;
use hermes_logging_components::traits::logger::CanLog;

use crate::chain::traits::queries::chain_status::CanQueryChainStatus;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::relay::impls::packet_relayers::general::filter_relayer::FilterRelayer;
use crate::relay::impls::packet_relayers::general::full_relay::{
    LogRelayPacketAction, PerformFullRelay,
};
use crate::relay::impls::packet_relayers::general::lock::{
    LockPacketRelayer, LogSkipRelayLockedPacket,
};
use crate::relay::impls::packet_relayers::general::log::{LogRelayPacketStatus, LoggerRelayer};
use crate::relay::impls::packet_relayers::skip_cleared::SkipClearedPacket;
use crate::relay::traits::chains::{HasRelayChains, HasRelayPacketType};
use crate::relay::traits::packet_filter::CanFilterRelayPackets;
use crate::relay::traits::packet_lock::HasPacketLock;
use crate::relay::traits::packet_relayer::{PacketRelayer, PacketRelayerComponent};
use crate::relay::traits::packet_relayers::ack_packet::CanRelayAckPacket;
use crate::relay::traits::packet_relayers::receive_packet::CanRelayReceivePacket;
use crate::relay::traits::packet_relayers::timeout_unordered_packet::CanRelayTimeoutUnorderedPacket;

pub struct DefaultPacketRelayer;

#[cgp_provider(PacketRelayerComponent)]
impl<Relay, SrcChain, DstChain> PacketRelayer<Relay> for DefaultPacketRelayer
where
    Relay: CanRelayAckPacket
        + CanRelayReceivePacket
        + CanRelayTimeoutUnorderedPacket
        + HasRelayPacketType
        + HasPacketLock
        + CanFilterRelayPackets
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + for<'a> CanLog<LogRelayPacketAction<'a, Relay>>
        + for<'a> CanLog<LogRelayPacketStatus<'a, Relay>>
        + for<'a> CanLog<LogSkipRelayLockedPacket<'a, Relay>>
        + CanRaiseAsyncError<SrcChain::Error>
        + CanRaiseAsyncError<DstChain::Error>,
    SrcChain:
        CanQueryChainStatus + CanQueryPacketIsCleared<DstChain> + CanReadPacketFields<DstChain>,
    DstChain: CanQueryChainStatus
        + HasWriteAckEvent<SrcChain>
        + CanBuildPacketFromWriteAck<SrcChain>
        + CanQueryPacketIsReceived<SrcChain>,
{
    async fn relay_packet(relay: &Relay, packet: &Relay::Packet) -> Result<(), Relay::Error> {
        <LockPacketRelayer<LoggerRelayer<FilterRelayer<SkipClearedPacket<PerformFullRelay>>>>>::relay_packet(
            relay, packet,
        )
        .await
    }
}
