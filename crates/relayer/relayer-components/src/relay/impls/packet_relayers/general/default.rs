use cgp::prelude::CanRaiseError;
use hermes_chain_components::traits::packet::fields::CanReadOutgoingPacketFields;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_chain_components::traits::types::timestamp::HasTimeoutType;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;

use crate::chain::traits::queries::chain_status::CanQueryChainStatus;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::relay::impls::packet_relayers::general::filter_relayer::FilterRelayer;
use crate::relay::impls::packet_relayers::general::full_relay::{
    FullCycleRelayer, LogRelayPacketAction,
};
use crate::relay::impls::packet_relayers::general::lock::{
    LockPacketRelayer, LogSkipRelayLockedPacket,
};
use crate::relay::impls::packet_relayers::general::log::{LogRelayPacketStatus, LoggerRelayer};
use crate::relay::traits::chains::{HasRelayChains, HasRelayPacketType};
use crate::relay::traits::packet_filter::CanFilterRelayPackets;
use crate::relay::traits::packet_lock::HasPacketLock;
use crate::relay::traits::packet_relayer::PacketRelayer;
use crate::relay::traits::packet_relayers::ack_packet::CanRelayAckPacket;
use crate::relay::traits::packet_relayers::receive_packet::CanRelayReceivePacket;
use crate::relay::traits::packet_relayers::timeout_unordered_packet::CanRelayTimeoutUnorderedPacket;

pub struct DefaultPacketRelayer;

impl<Relay, SrcChain, DstChain> PacketRelayer<Relay> for DefaultPacketRelayer
where
    Relay: CanRelayAckPacket
        + CanRelayReceivePacket
        + CanRelayTimeoutUnorderedPacket
        + HasRelayPacketType
        + HasLogger
        + HasPacketLock
        + CanFilterRelayPackets
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + CanRaiseError<SrcChain::Error>
        + CanRaiseError<DstChain::Error>,
    SrcChain: CanQueryChainStatus + CanReadOutgoingPacketFields<DstChain>,
    DstChain: CanQueryChainStatus
        + HasWriteAckEvent<Relay::SrcChain>
        + HasChannelIdType<SrcChain>
        + HasPortIdType<SrcChain>
        + HasTimeoutType,
    Relay::Logger: for<'a> CanLog<LogRelayPacketAction<'a, Relay>>
        + for<'a> CanLog<LogRelayPacketStatus<'a, Relay>>
        + for<'a> CanLog<LogSkipRelayLockedPacket<'a, Relay>>,
{
    async fn relay_packet(relay: &Relay, packet: &Relay::Packet) -> Result<(), Relay::Error> {
        <LockPacketRelayer<LoggerRelayer<FilterRelayer<FullCycleRelayer>>>>::relay_packet(
            relay, packet,
        )
        .await
    }
}
