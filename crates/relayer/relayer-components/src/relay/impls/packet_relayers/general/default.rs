use cgp::prelude::*;
use hermes_chain_components::traits::{
    CanBuildPacketFromWriteAck, CanQueryPacketIsCleared, CanQueryPacketIsReceived,
    CanReadPacketFields,
};
use hermes_logging_components::traits::CanLog;

use crate::chain::traits::{CanQueryChainStatus, HasWriteAckEvent};
use crate::relay::impls::{
    FilterRelayer, LockPacketRelayer, LogRelayPacketAction, LogRelayPacketStatus,
    LogSkipRelayLockedPacket, LoggerRelayer, PerformFullRelay, SkipClearedPacket,
};
use crate::relay::traits::{
    CanFilterRelayPackets, CanRelayAckPacket, CanRelayReceivePacket,
    CanRelayTimeoutUnorderedPacket, HasPacketLock, HasRelayChains, HasRelayPacketType,
    PacketRelayer, PacketRelayerComponent,
};

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
