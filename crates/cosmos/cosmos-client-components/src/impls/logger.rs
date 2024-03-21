use cgp_core::{Async, HasErrorType};
use core::fmt::{Debug, Display};
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::log::traits::logger::Logger;
use hermes_relayer_components::relay::impls::packet_clearers::receive_packet::LogClearPacketError;
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::LogRelayPacketAction;
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use hermes_relayer_components::relay::impls::packet_relayers::general::log::{
    LogRelayPacketStatus, RelayPacketStatus,
};
use hermes_relayer_components::relay::impls::update_client::skip::LogSkipBuildUpdateClientMessage;
use hermes_relayer_components::relay::impls::update_client::wait::LogWaitUpdateClientHeightStatus;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::target::ChainTarget;
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use hermes_relayer_components::transaction::impls::poll_tx_response::{
    LogRetryQueryTxResponse, TxNoResponseError,
};
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_relayer_components_extra::batch::worker::LogBatchWorker;
use tracing::{debug, error, info, trace};

pub struct HandleCosmosLogs;

impl<Logging> Logger<Logging, ()> for HandleCosmosLogs
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &()) {
        info!("{message}");
    }
}

impl<'a, Logging, Chain> Logger<Logging, LogSendMessagesWithSignerAndNonce<'a, Chain>>
    for HandleCosmosLogs
where
    Logging: Async,
    Chain: HasSignerType + HasNonceType + HasMessageType + HasChainId,
    Chain::Signer: Debug,
    Chain::Nonce: Debug,
{
    async fn log(
        _logging: &Logging,
        message: &str,
        details: &LogSendMessagesWithSignerAndNonce<'a, Chain>,
    ) {
        trace!(
            chain_id = %details.chain.chain_id(),
            nonce = ?details.nonce,
            signer = ?details.signer,
            "{message}",
        );
    }
}

impl<'a, Logging, Chain> Logger<Logging, TxNoResponseError<'a, Chain>> for HandleCosmosLogs
where
    Logging: Async,
    Chain: HasTransactionHashType + HasChainId,
    Chain::TxHash: Display,
{
    async fn log(_logging: &Logging, message: &str, details: &TxNoResponseError<'a, Chain>) {
        error!(
            chain_id = %details.chain.chain_id(),
            tx_hash = %details.tx_hash,
            wait_timeout = ?details.wait_timeout,
            elapsed = ?details.elapsed,
            "{message}",
        );
    }
}

impl<'a, Logging, Chain> Logger<Logging, LogRetryQueryTxResponse<'a, Chain>> for HandleCosmosLogs
where
    Logging: Async,
    Chain: HasTransactionHashType + HasChainId + HasErrorType,
    Chain::TxHash: Display,
    Chain::Error: Debug,
{
    async fn log(_logging: &Logging, message: &str, details: &LogRetryQueryTxResponse<'a, Chain>) {
        debug!(
            chain_id = %details.chain.chain_id(),
            tx_hash = %details.tx_hash,
            elapsed = ?details.elapsed,
            error = ?details.error,
            "{message}",
        );
    }
}

impl<'a, Logging, Relay> Logger<Logging, LogSkipRelayLockedPacket<'a, Relay>> for HandleCosmosLogs
where
    Logging: Async,
    Relay: HasRelayChains,
    Relay::Packet: Display,
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

impl<'a, Logging, Relay> Logger<Logging, LogRelayPacketAction<'a, Relay>> for HandleCosmosLogs
where
    Logging: Async,
    Relay: HasRelayChains,
    Relay::Packet: Display,
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

impl<'a, Logging, Relay> Logger<Logging, LogClearPacketError<'a, Relay>> for HandleCosmosLogs
where
    Logging: Async,
    Relay: HasRelayChains,
    Relay::Packet: Display,
    Relay::SrcChain: HasChainId,
    Relay::DstChain: HasChainId,
{
    async fn log(_logging: &Logging, message: &str, details: &LogClearPacketError<'a, Relay>) {
        error!(
            packet = %details.packet,
            src_chain_id = %details.relay.src_chain().chain_id(),
            dst_chain_id = %details.relay.dst_chain().chain_id(),
            clear_action = ?details.clear_action,
            error = ?details.error,
            "{message}",
        );
    }
}

impl<'a, Logging, Relay> Logger<Logging, LogRelayPacketStatus<'a, Relay>> for HandleCosmosLogs
where
    Logging: Async,
    Relay: HasRelayChains,
    Relay::Packet: Display,
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

impl<'a, Logging, Relay, Target> Logger<Logging, LogSkipBuildUpdateClientMessage<'a, Relay, Target>>
    for HandleCosmosLogs
where
    Logging: Async,
    Relay: HasRelayChains,
    Target: ChainTarget<Relay>,
    Target::TargetChain: HasChainId,
    Target::CounterpartyChain: HasChainId,
{
    async fn log(
        _logging: &Logging,
        message: &str,
        details: &LogSkipBuildUpdateClientMessage<'a, Relay, Target>,
    ) {
        trace!(
            target_chain_id = %Target::target_chain(details.relay).chain_id(),
            counterparty_chain_id = %Target::counterparty_chain(details.relay).chain_id(),
            target_height = %details.target_height,
            "{message}",
        );
    }
}

impl<'a, Logging, Relay, Target> Logger<Logging, LogWaitUpdateClientHeightStatus<'a, Relay, Target>>
    for HandleCosmosLogs
where
    Logging: Async,
    Relay: HasRelayChains,
    Target: ChainTarget<Relay>,
    Target::TargetChain: HasChainId,
    Target::CounterpartyChain: HasChainId,
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
                    target_chain_id = %Target::target_chain(relay).chain_id(),
                    counterparty_chain_id = %Target::counterparty_chain(relay).chain_id(),
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
                    target_chain_id = %Target::target_chain(relay).chain_id(),
                    counterparty_chain_id = %Target::counterparty_chain(relay).chain_id(),
                    %target_height,
                    %current_height,
                    "{message}",
                );
            }
        }
    }
}

impl<'a, Logging, Relay, Target> Logger<Logging, LogBatchWorker<'a, Relay, Target>>
    for HandleCosmosLogs
where
    Logging: Async,
    Relay: HasRelayChains,
    Target: ChainTarget<Relay>,
    Target::TargetChain: HasChainId,
    Target::CounterpartyChain: HasChainId,
{
    async fn log(_logging: &Logging, message: &str, details: &LogBatchWorker<'a, Relay, Target>) {
        trace!(
            target_chain_id = %Target::target_chain(details.relay).chain_id(),
            counterparty_chain_id = %Target::counterparty_chain(details.relay).chain_id(),
            details = %details.details,
            "{message}",
        );
    }
}
