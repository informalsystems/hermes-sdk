use core::fmt::{Debug, Display};

use cgp::prelude::*;
use hermes_logging_components::traits::logger::{Logger, LoggerComponent};
use hermes_logging_components::types::level::LogLevel;
use hermes_relayer_components::birelay::impls::LogAutoBiRelay;
use hermes_relayer_components::birelay::traits::{HasBiRelayTypes, HasTwoWayRelay};
use hermes_relayer_components::chain::traits::{HasChainId, HasClientIdType, HasHeightType};
use hermes_relayer_components::relay::impls::{
    LogAutoRelayWithHeights, LogClientUpdateMessage, LogRelayPacketAction, LogRelayPacketStatus,
    LogSkipBuildUpdateClientMessage, LogSkipRelayLockedPacket, LogWaitUpdateClientHeightStatus,
    RelayPacketStatus,
};
use hermes_relayer_components::relay::traits::{
    HasDstChain, HasRelayChains, HasSrcChain, HasTargetChains, HasTargetClientIds, PacketOf,
    RelayTarget,
};
use hermes_relayer_components_extra::batch::worker::LogBatchWorker;
use tracing::{debug, error, info, trace};

use crate::contexts::logger::TracingLogger;

#[cgp_provider(LoggerComponent)]
impl<'a, Relay> Logger<Relay, LogSkipRelayLockedPacket<'a, Relay>> for TracingLogger
where
    Relay: HasRelayChains,
    PacketOf<Relay>: Display,
    Relay::SrcChain: HasChainId,
    Relay::DstChain: HasChainId,
{
    async fn log(relay: &Relay, message: &str, details: &LogSkipRelayLockedPacket<'a, Relay>) {
        trace!(
            target: "hermes::packet",
            packet = %details.packet,
            src_chain_id = %relay.src_chain().chain_id(),
            dst_chain_id = %relay.dst_chain().chain_id(),
            "{message}",
        );
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Relay> Logger<Relay, LogRelayPacketAction<'a, Relay>> for TracingLogger
where
    Relay: HasRelayChains,
    PacketOf<Relay>: Display,
    Relay::SrcChain: HasChainId,
    Relay::DstChain: HasChainId,
{
    async fn log(relay: &Relay, message: &str, details: &LogRelayPacketAction<'a, Relay>) {
        debug!(
            target: "hermes::packet",
            packet = %details.packet,
            src_chain_id = %relay.src_chain().chain_id(),
            dst_chain_id = %relay.dst_chain().chain_id(),
            relay_progress = ?details.relay_progress,
            "{message}",
        );
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Relay> Logger<Relay, LogRelayPacketStatus<'a, Relay>> for TracingLogger
where
    Relay: HasRelayChains,
    PacketOf<Relay>: Display,
    Relay::SrcChain: HasChainId,
    Relay::DstChain: HasChainId,
{
    async fn log(relay: &Relay, message: &str, details: &LogRelayPacketStatus<'a, Relay>) {
        match details.relay_status {
            RelayPacketStatus::Start => {
                trace!(
                    target: "hermes::packet",
                    packet = %details.packet,
                    src_chain_id = %relay.src_chain().chain_id(),
                    dst_chain_id = %relay.dst_chain().chain_id(),
                    relay_status = "start",
                    "{message}",
                );
            }
            RelayPacketStatus::Successful => {
                trace!(
                    target: "hermes",
                    packet = %details.packet,
                    src_chain_id = %relay.src_chain().chain_id(),
                    dst_chain_id = %relay.dst_chain().chain_id(),
                    relay_status = "successful",
                    "{message}",
                );
            }
            RelayPacketStatus::Error { error } => {
                error!(
                    target: "hermes::packet",
                    packet = %details.packet,
                    src_chain_id = %relay.src_chain().chain_id(),
                    dst_chain_id = %relay.dst_chain().chain_id(),
                    ?error,
                    "{message}",
                );
            }
        }
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Relay, Target> Logger<Relay, LogSkipBuildUpdateClientMessage<'a, Relay, Target>>
    for TracingLogger
where
    Relay: HasTargetChains<Target>,
    Target: RelayTarget,
    Relay::TargetChain: HasChainId,
    Relay::CounterpartyChain: HasChainId + HasHeightType,
{
    async fn log(
        relay: &Relay,
        message: &str,
        details: &LogSkipBuildUpdateClientMessage<'a, Relay, Target>,
    ) {
        trace!(
            target: "hermes::update_client",
            target_chain_id = %relay.target_chain().chain_id(),
            counterparty_chain_id = %relay.counterparty_chain().chain_id(),
            target_height = %details.target_height,
            "{message}",
        );
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Relay, Target> Logger<Relay, LogWaitUpdateClientHeightStatus<'a, Relay, Target>>
    for TracingLogger
where
    Relay: HasTargetChains<Target>,
    Target: RelayTarget,
    Relay::TargetChain: HasChainId,
    Relay::CounterpartyChain: HasChainId + HasHeightType,
{
    async fn log(
        relay: &Relay,
        message: &str,
        details: &LogWaitUpdateClientHeightStatus<'a, Relay, Target>,
    ) {
        match details {
            LogWaitUpdateClientHeightStatus::Waiting { target_height } => {
                trace!(
                    target: "hermes::update_client",
                    target_chain_id = %relay.target_chain().chain_id(),
                    counterparty_chain_id = %relay.counterparty_chain().chain_id(),
                    %target_height,
                    "{message}",
                );
            }
            LogWaitUpdateClientHeightStatus::HeightReached {
                target_height,
                current_height,
            } => {
                trace!(
                    target: "hermes::update_client",
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
impl<'a, Relay, Target> Logger<Relay, LogBatchWorker<'a, Target>> for TracingLogger
where
    Relay: HasTargetChains<Target>,
    Target: RelayTarget,
    Relay::TargetChain: HasChainId,
    Relay::CounterpartyChain: HasChainId,
{
    async fn log(relay: &Relay, message: &str, details: &LogBatchWorker<'a, Target>) {
        match details.log_level {
            LogLevel::Error => {
                error!(
                    target: "hermes::batch",
                    target_chain_id = %relay.target_chain().chain_id(),
                    counterparty_chain_id = %relay.counterparty_chain().chain_id(),
                    details = %details.details,
                    "{message}",
                );
            }
            _ => {
                trace!(
                    target: "hermes::batch",
                    target_chain_id = %relay.target_chain().chain_id(),
                    counterparty_chain_id = %relay.counterparty_chain().chain_id(),
                    details = %details.details,
                    "{message}",
                );
            }
        }
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Logging, Relay, Target> Logger<Logging, LogClientUpdateMessage<'a, Relay, Target>>
    for TracingLogger
where
    Logging: Async,
    Relay: HasTargetChains<Target>,
    Target: RelayTarget,
    Relay::TargetChain: HasChainId + HasClientIdType<Relay::CounterpartyChain>,
    Relay::CounterpartyChain: HasChainId + HasHeightType,
{
    async fn log(
        _logging: &Logging,
        message: &str,
        details: &LogClientUpdateMessage<'a, Relay, Target>,
    ) {
        debug!(
            target: "hermes::update_client",
            target_chain_id = %details.relay.target_chain().chain_id(),
            counterparty_chain_id = %details.relay.counterparty_chain().chain_id(),
            client_id = %details.client_id,
            target_height = %details.target_height,
            "{message}",
        );
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Logging, BiRelay> Logger<Logging, LogAutoBiRelay<'a, BiRelay>> for TracingLogger
where
    Logging: Async,
    BiRelay: HasBiRelayTypes<
            ChainA: HasHeightType<Height: Debug> + HasChainId,
            ChainB: HasHeightType<Height: Debug> + HasChainId,
            RelayAToB: HasSrcChain + HasDstChain,
        > + HasTwoWayRelay,
{
    async fn log(_logging: &Logging, message: &str, details: &LogAutoBiRelay<'a, BiRelay>) {
        info!(
            target: "hermes::relay",
            chain_id_a = %details.bi_relay.relay_a_to_b().src_chain().chain_id(),
            chain_id_b = %details.bi_relay.relay_a_to_b().dst_chain().chain_id(),
            start_height_a = %details.start_height_a,
            start_height_b = %details.start_height_b,
            end_height_a = ?details.end_height_a,
            end_height_b = ?details.end_height_b,
            clear_past_blocks = ?details.clear_past_blocks,
            stop_after_blocks = ?details.stop_after_blocks,
            "{message}",
        );
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Logging, Relay, Target> Logger<Logging, LogAutoRelayWithHeights<'a, Relay, Target>>
    for TracingLogger
where
    Logging: Async,
    Relay: HasTargetChains<Target> + HasTargetClientIds<Target>,
    Target: RelayTarget,
    Relay::TargetChain: HasChainId + HasHeightType,
    Relay::CounterpartyChain: HasChainId,
{
    async fn log(
        _logging: &Logging,
        message: &str,
        details: &LogAutoRelayWithHeights<'a, Relay, Target>,
    ) {
        info!(
            target: "hermes::relay",
            target_chain_id = %details.relay.target_chain().chain_id(),
            counterparty_chain_id = %details.relay.counterparty_chain().chain_id(),
            target_client_id = %details.relay.target_client_id(),
            counterparty_client_id = %details.relay.counterparty_client_id(),
            start_height = %details.start_height,
            end_height = ?details.end_height,
            "{message}",
        );
    }
}
