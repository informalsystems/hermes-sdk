use core::fmt::Display;

use cgp::prelude::*;
use hermes_logging_components::traits::logger::{Logger, LoggerComponent};
use hermes_logging_components::types::level::LogLevel;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::LogRelayPacketAction;
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use hermes_relayer_components::relay::impls::packet_relayers::general::log::{
    LogRelayPacketStatus, RelayPacketStatus,
};
use hermes_relayer_components::relay::impls::update_client::skip::LogSkipBuildUpdateClientMessage;
use hermes_relayer_components::relay::impls::update_client::wait::LogWaitUpdateClientHeightStatus;
use hermes_relayer_components::relay::traits::chains::{HasRelayChains, PacketOf};
use hermes_relayer_components::relay::traits::target::{HasTargetChains, RelayTarget};
use hermes_relayer_components_extra::batch::worker::LogBatchWorker;
use tracing::{error, trace};

use crate::contexts::logger::TracingLogger;

#[cgp_provider(LoggerComponent)]
impl<'a, Logging, Relay> Logger<Logging, LogSkipRelayLockedPacket<'a, Relay>> for TracingLogger
where
    Logging: Async,
    Relay: HasRelayChains,
    PacketOf<Relay>: Display,
    Relay::SrcChain: HasChainId,
    Relay::DstChain: HasChainId,
{
    async fn log(_logging: &Logging, message: &str, details: &LogSkipRelayLockedPacket<'a, Relay>) {
        trace!(
            packet = %details.packet,
            src_chain_id = %details.relay.src_chain().chain_id(),
            dst_chain_id = %details.relay.dst_chain().chain_id(),
            "{message}",
        );
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Logging, Relay> Logger<Logging, LogRelayPacketAction<'a, Relay>> for TracingLogger
where
    Logging: Async,
    Relay: HasRelayChains,
    PacketOf<Relay>: Display,
    Relay::SrcChain: HasChainId,
    Relay::DstChain: HasChainId,
{
    async fn log(_logging: &Logging, message: &str, details: &LogRelayPacketAction<'a, Relay>) {
        trace!(
            packet = %details.packet,
            src_chain_id = %details.relay.src_chain().chain_id(),
            dst_chain_id = %details.relay.dst_chain().chain_id(),
            relay_progress = ?details.relay_progress,
            "{message}",
        );
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Logging, Relay> Logger<Logging, LogRelayPacketStatus<'a, Relay>> for TracingLogger
where
    Logging: Async,
    Relay: HasRelayChains,
    PacketOf<Relay>: Display,
    Relay::SrcChain: HasChainId,
    Relay::DstChain: HasChainId,
{
    async fn log(_logging: &Logging, message: &str, details: &LogRelayPacketStatus<'a, Relay>) {
        match details.relay_status {
            RelayPacketStatus::Start => {
                trace!(
                    packet = %details.packet,
                    src_chain_id = %details.relay.src_chain().chain_id(),
                    dst_chain_id = %details.relay.dst_chain().chain_id(),
                    relay_status = "start",
                    "{message}",
                );
            }
            RelayPacketStatus::Successful => {
                trace!(
                    packet = %details.packet,
                    src_chain_id = %details.relay.src_chain().chain_id(),
                    dst_chain_id = %details.relay.dst_chain().chain_id(),
                    relay_status = "successful",
                    "{message}",
                );
            }
            RelayPacketStatus::Error { error } => {
                error!(
                    packet = %details.packet,
                    src_chain_id = %details.relay.src_chain().chain_id(),
                    dst_chain_id = %details.relay.dst_chain().chain_id(),
                    ?error,
                    "{message}",
                );
            }
        }
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Logging, Relay, Target> Logger<Logging, LogSkipBuildUpdateClientMessage<'a, Relay, Target>>
    for TracingLogger
where
    Logging: Async,
    Relay: HasTargetChains<Target>,
    Target: RelayTarget,
    Relay::TargetChain: HasChainId,
    Relay::CounterpartyChain: HasChainId + HasHeightType,
{
    async fn log(
        _logging: &Logging,
        message: &str,
        details: &LogSkipBuildUpdateClientMessage<'a, Relay, Target>,
    ) {
        trace!(
            target_chain_id = %details.relay.target_chain().chain_id(),
            counterparty_chain_id = %details.relay.counterparty_chain().chain_id(),
            target_height = %details.target_height,
            "{message}",
        );
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Logging, Relay, Target> Logger<Logging, LogWaitUpdateClientHeightStatus<'a, Relay, Target>>
    for TracingLogger
where
    Logging: Async,
    Relay: HasTargetChains<Target>,
    Target: RelayTarget,
    Relay::TargetChain: HasChainId,
    Relay::CounterpartyChain: HasChainId + HasHeightType,
{
    async fn log(
        _logging: &Logging,
        message: &str,
        details: &LogWaitUpdateClientHeightStatus<'a, Relay, Target>,
    ) {
        match details {
            LogWaitUpdateClientHeightStatus::Waiting {
                relay,
                target_height,
            } => {
                trace!(
                    target_chain_id = %relay.target_chain().chain_id(),
                    counterparty_chain_id = %relay.counterparty_chain().chain_id(),
                    %target_height,
                    "{message}",
                );
            }
            LogWaitUpdateClientHeightStatus::HeightReached {
                relay,
                target_height,
                current_height,
            } => {
                trace!(
                    target_chain_id = %relay.target_chain().chain_id(),
                    counterparty_chain_id = %relay.counterparty_chain().chain_id(),
                    %target_height,
                    %current_height,
                    "{message}",
                );
            }
        }
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Logging, Relay, Target> Logger<Logging, LogBatchWorker<'a, Relay, Target>>
    for TracingLogger
where
    Logging: Async,
    Relay: HasTargetChains<Target>,
    Target: RelayTarget,
    Relay::TargetChain: HasChainId,
    Relay::CounterpartyChain: HasChainId,
{
    async fn log(_logging: &Logging, message: &str, details: &LogBatchWorker<'a, Relay, Target>) {
        match details.log_level {
            LogLevel::Error => {
                error!(
                    target_chain_id = %details.relay.target_chain().chain_id(),
                    counterparty_chain_id = %details.relay.counterparty_chain().chain_id(),
                    details = %details.details,
                    "{message}",
                );
            }
            _ => {
                trace!(
                    target_chain_id = %details.relay.target_chain().chain_id(),
                    counterparty_chain_id = %details.relay.counterparty_chain().chain_id(),
                    details = %details.details,
                    "{message}",
                );
            }
        }
    }
}
