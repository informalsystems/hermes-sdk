use cgp_core::prelude::*;
use hermes_logging_components::impls::delegate::DelegateLogger;
use hermes_logging_components::impls::global::GetGlobalLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetter, HasLoggerType, LoggerGetterComponent, ProvideLoggerType,
};
use hermes_logging_components::traits::logger::{CanLog, LoggerComponent};
use hermes_relayer_components::relay::impls::packet_clearers::receive_packet::LogClearPacketError;
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::LogRelayPacketAction;
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use hermes_relayer_components::relay::impls::packet_relayers::general::log::LogRelayPacketStatus;
use hermes_relayer_components::relay::impls::update_client::skip::LogSkipBuildUpdateClientMessage;
use hermes_relayer_components::relay::impls::update_client::wait::LogWaitUpdateClientHeightStatus;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use hermes_relayer_components::transaction::impls::poll_tx_response::{
    LogRetryQueryTxResponse, TxNoResponseError,
};
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_relayer_components_extra::batch::worker::LogBatchWorker;
use hermes_tracing_logging_components::contexts::logger::TracingLogger;

use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;

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
        (): TracingLogger,
    }
}

impl<'a> DelegateComponent<LogSendMessagesWithSignerAndNonce<'a, CosmosChain>>
    for CosmosLogHandlers
{
    type Delegate = TracingLogger;
}

impl<'a, Chain> DelegateComponent<TxNoResponseError<'a, Chain>> for CosmosLogHandlers
where
    Chain: HasTransactionHashType,
{
    type Delegate = TracingLogger;
}

impl<'a, Chain> DelegateComponent<LogRetryQueryTxResponse<'a, Chain>> for CosmosLogHandlers
where
    Chain: HasTransactionHashType + HasErrorType,
{
    type Delegate = TracingLogger;
}

impl<'a> DelegateComponent<LogSkipRelayLockedPacket<'a, CosmosRelay>> for CosmosLogHandlers {
    type Delegate = TracingLogger;
}

impl<'a> DelegateComponent<LogRelayPacketAction<'a, CosmosRelay>> for CosmosLogHandlers {
    type Delegate = TracingLogger;
}

impl<'a> DelegateComponent<LogClearPacketError<'a, CosmosRelay>> for CosmosLogHandlers {
    type Delegate = TracingLogger;
}

impl<'a> DelegateComponent<LogRelayPacketStatus<'a, CosmosRelay>> for CosmosLogHandlers {
    type Delegate = TracingLogger;
}

impl<'a> DelegateComponent<LogSkipBuildUpdateClientMessage<'a, CosmosRelay, SourceTarget>>
    for CosmosLogHandlers
{
    type Delegate = TracingLogger;
}

impl<'a> DelegateComponent<LogSkipBuildUpdateClientMessage<'a, CosmosRelay, DestinationTarget>>
    for CosmosLogHandlers
{
    type Delegate = TracingLogger;
}

impl<'a> DelegateComponent<LogWaitUpdateClientHeightStatus<'a, CosmosRelay, SourceTarget>>
    for CosmosLogHandlers
{
    type Delegate = TracingLogger;
}

impl<'a> DelegateComponent<LogWaitUpdateClientHeightStatus<'a, CosmosRelay, DestinationTarget>>
    for CosmosLogHandlers
{
    type Delegate = TracingLogger;
}

impl<'a> DelegateComponent<LogBatchWorker<'a, CosmosRelay, SourceTarget>> for CosmosLogHandlers {
    type Delegate = TracingLogger;
}

impl<'a> DelegateComponent<LogBatchWorker<'a, CosmosRelay, DestinationTarget>>
    for CosmosLogHandlers
{
    type Delegate = TracingLogger;
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
