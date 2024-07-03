use cgp_core::prelude::*;
use hermes_logging_components::impls::delegate::DelegateLogger;
use hermes_logging_components::impls::global::GetGlobalLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetter, HasLoggerType, LoggerGetterComponent, ProvideLoggerType,
};
use hermes_logging_components::traits::logger::{CanLog, LoggerComponent};
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::relay::impls::packet_clearers::receive_packet::LogClearPacketError;
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::LogRelayPacketAction;
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use hermes_relayer_components::relay::impls::packet_relayers::general::log::LogRelayPacketStatus;
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
use hermes_tracing_logging_components::contexts::logger::TracingLogger;

use crate::contexts::chain::CosmosChain;

pub struct CosmosLogger;

pub struct CosmosLoggerComponents;

pub struct CosmosLogHandlers;

impl HasComponents for CosmosLogger {
    type Components = CosmosLoggerComponents;
}

delegate_components! {
    CosmosLoggerComponents {
        LoggerComponent: DelegateLogger<CosmosLogHandlers>,
    }
}

pub trait CanUseCosmosLogger:
    for<'a> CanLog<LogSendMessagesWithSignerAndNonce<'a, CosmosChain>>
    + for<'a> CanLog<TxNoResponseError<'a, CosmosChain>>
{
}

impl CanUseCosmosLogger for CosmosLogger {}

delegate_components! {
    CosmosLogHandlers {
        [
            (),
            <'a, Chain: HasSignerType + HasNonceType + HasMessageType,>
                LogSendMessagesWithSignerAndNonce<'a, Chain>,
            <'a, Chain: HasTransactionHashType>
                TxNoResponseError<'a, Chain>,
            <'a, Chain: HasTransactionHashType + HasErrorType>
                LogRetryQueryTxResponse<'a, Chain>,
            <'a, Relay: HasRelayChains>
                LogSkipRelayLockedPacket<'a, Relay>,
            <'a, Relay: HasRelayChains>
                LogRelayPacketAction<'a, Relay>,
            <'a, Relay: HasRelayChains>
                LogClearPacketError<'a, Relay>,
            <'a, Relay: HasRelayChains>
                LogRelayPacketStatus<'a, Relay>,
            <'a, Relay: HasRelayChains, Target: ChainTarget<Relay>>
                LogSkipBuildUpdateClientMessage<'a, Relay, Target>,
            <'a, Relay: HasRelayChains, Target: ChainTarget<Relay>>
                LogWaitUpdateClientHeightStatus<'a, Relay, Target>,
            <'a, Relay: HasRelayChains, Target: ChainTarget<Relay>>
                LogBatchWorker<'a, Relay, Target>,
        ]: TracingLogger,
    }
}

pub struct ProvideCosmosLogger;

delegate_components! {
    ProvideCosmosLogger {
        LoggerGetterComponent: GetGlobalLogger,
    }
}

impl<Context> ProvideLoggerType<Context> for ProvideCosmosLogger
where
    Context: Async,
{
    type Logger = CosmosLogger;
}

impl<Context> GlobalLoggerGetter<Context> for ProvideCosmosLogger
where
    Context: HasLoggerType<Logger = CosmosLogger>,
{
    fn global_logger() -> &'static CosmosLogger {
        &CosmosLogger
    }
}
