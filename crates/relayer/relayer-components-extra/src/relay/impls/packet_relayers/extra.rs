use cgp::prelude::CanRaiseError;
use hermes_chain_type_components::traits::types::timeout::HasTimeoutType;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_relayer_components::chain::traits::packet::fields::CanReadOutgoingPacketFields;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::error::impls::error::MaxRetryExceededError;
use hermes_relayer_components::error::traits::retry::{HasMaxErrorRetry, HasRetryableError};
use hermes_relayer_components::relay::impls::packet_relayers::general::filter_relayer::FilterRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::{
    FullCycleRelayer, LogRelayPacketAction,
};
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::{
    LockPacketRelayer, LogSkipRelayLockedPacket,
};
use hermes_relayer_components::relay::impls::packet_relayers::general::log::{
    LogRelayPacketStatus, LoggerRelayer,
};
use hermes_relayer_components::relay::traits::chains::{HasRelayChains, HasRelayPacketType};
use hermes_relayer_components::relay::traits::packet_filter::CanFilterRelayPackets;
use hermes_relayer_components::relay::traits::packet_lock::HasPacketLock;
use hermes_relayer_components::relay::traits::packet_relayer::PacketRelayer;
use hermes_relayer_components::relay::traits::packet_relayers::ack_packet::CanRelayAckPacket;
use hermes_relayer_components::relay::traits::packet_relayers::receive_packet::CanRelayReceivePacket;
use hermes_relayer_components::relay::traits::packet_relayers::timeout_unordered_packet::CanRelayTimeoutUnorderedPacket;

use crate::relay::impls::packet_relayers::retry::RetryRelayer;

pub struct ExtraPacketRelayer;

impl<Relay, SrcChain, DstChain> PacketRelayer<Relay> for ExtraPacketRelayer
where
    Relay: CanRelayAckPacket
        + CanRelayReceivePacket
        + CanRelayTimeoutUnorderedPacket
        + HasLogger
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasRelayPacketType
        + CanFilterRelayPackets
        + HasPacketLock
        + HasMaxErrorRetry
        + HasRetryableError
        + CanRaiseError<SrcChain::Error>
        + CanRaiseError<DstChain::Error>
        + for<'a> CanRaiseError<MaxRetryExceededError<'a, Relay>>,
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
        <LockPacketRelayer<LoggerRelayer<FilterRelayer<RetryRelayer<FullCycleRelayer>>>>>::
            relay_packet(relay, packet).await
    }
}