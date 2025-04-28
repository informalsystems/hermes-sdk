use cgp::prelude::*;
use hermes_logging_components::traits::CanLog;
use hermes_relayer_components::chain::traits::{
    CanBuildPacketFromWriteAck, CanQueryChainStatus, CanQueryPacketIsCleared,
    CanQueryPacketIsReceived, CanReadPacketFields, HasWriteAckEvent,
};
use hermes_relayer_components::error::traits::{
    CanPerformRetry, HasMaxErrorRetry, HasRetryableError,
};
use hermes_relayer_components::relay::impls::{
    FilterRelayer, LockPacketRelayer, LogRelayPacketAction, LogRelayPacketStatus,
    LogSkipRelayLockedPacket, LoggerRelayer, PerformFullRelay,
};
use hermes_relayer_components::relay::traits::{
    CanFilterRelayPackets, CanRelayAckPacket, CanRelayReceivePacket,
    CanRelayTimeoutUnorderedPacket, HasPacketLock, HasRelayChains, HasRelayPacketType,
    PacketRelayer, PacketRelayerComponent,
};

use crate::relay::impls::packet_relayers::retry::RelayPacketWithRetry;

pub struct ExtraPacketRelayer;

#[cgp_provider(PacketRelayerComponent)]
impl<Relay, SrcChain, DstChain> PacketRelayer<Relay> for ExtraPacketRelayer
where
    Relay: CanRelayAckPacket
        + CanRelayReceivePacket
        + CanRelayTimeoutUnorderedPacket
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasRelayPacketType
        + CanFilterRelayPackets
        + HasPacketLock
        + HasMaxErrorRetry
        + CanPerformRetry
        + HasRetryableError
        + for<'a> CanLog<LogRelayPacketAction<'a, Relay>>
        + for<'a> CanLog<LogRelayPacketStatus<'a, Relay>>
        + for<'a> CanLog<LogSkipRelayLockedPacket<'a, Relay>>
        + CanRaiseAsyncError<SrcChain::Error>
        + CanRaiseAsyncError<DstChain::Error>,
    SrcChain:
        CanQueryChainStatus + CanQueryPacketIsCleared<DstChain> + CanReadPacketFields<DstChain>,
    DstChain: CanQueryChainStatus
        + HasWriteAckEvent<Relay::SrcChain>
        + CanBuildPacketFromWriteAck<Relay::SrcChain>
        + CanQueryPacketIsReceived<SrcChain>,
{
    async fn relay_packet(relay: &Relay, packet: &Relay::Packet) -> Result<(), Relay::Error> {
        <LockPacketRelayer<LoggerRelayer<FilterRelayer<RelayPacketWithRetry<PerformFullRelay>>>>>::
            relay_packet(relay, packet).await
    }
}
