use cgp_core::component::HasComponents;
use cgp_core::error::ErrorRaiser;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;

use crate::chain::traits::packet::from_write_ack::CanBuildPacketFromWriteAck;
use crate::chain::traits::queries::counterparty_chain_id::CanQueryCounterpartyChainId;
use crate::chain::traits::types::chain_id::HasChainId;
use crate::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use crate::components::default::closures::relay::ack_packet_relayer::UseDefaultAckPacketRelayer;
use crate::components::default::closures::relay::packet_relayer::UseDefaultPacketRelayer;
use crate::components::default::relay::DelegatesToDefaultRelayComponents;
use crate::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use crate::relay::traits::event_relayer::CanRelayEvent;
use crate::relay::traits::packet::HasRelayPacketFields;
use crate::relay::traits::packet_filter::PacketFilter;
use crate::relay::traits::packet_lock::HasPacketLock;
use crate::relay::traits::target::{DestinationTarget, SourceTarget};

pub trait CanUseDefaultEventRelayer: UseDefaultEventRelayer {}

pub trait UseDefaultEventRelayer:
    CanRelayEvent<SourceTarget> + CanRelayEvent<DestinationTarget> + CanRaiseRelayChainErrors
{
}

impl<Relay, SrcChain, DstChain, Components> UseDefaultEventRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasPacketLock
        + HasRelayPacketFields
        + HasLogger
        + UseDefaultAckPacketRelayer
        + UseDefaultPacketRelayer
        + HasComponents<Components = Components>,
    SrcChain: HasChainId
        + HasSendPacketEvent<Relay::DstChain>
        + CanQueryCounterpartyChainId<Relay::DstChain>,
    DstChain: HasChainId
        + CanQueryCounterpartyChainId<Relay::SrcChain>
        + CanBuildPacketFromWriteAck<Relay::SrcChain>,
    Components: DelegatesToDefaultRelayComponents
        + PacketFilter<Relay>
        + ErrorRaiser<Relay, SrcChain::Error>
        + ErrorRaiser<Relay, DstChain::Error>,
    Relay::Logger: for<'a> CanLog<LogSkipRelayLockedPacket<'a, Relay>>,
{
}
