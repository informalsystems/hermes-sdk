use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::logger::HandleCosmosLogs;
use hermes_relayer_components::log::impls::delegate::DelegateLogger;
use hermes_relayer_components::log::impls::global::GetGlobalLogger;
use hermes_relayer_components::log::traits::has_logger::{
    GlobalLoggerGetter, HasLoggerType, LoggerGetterComponent, ProvideLoggerType,
};
use hermes_relayer_components::log::traits::logger::{CanLog, LoggerComponent};
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
use hermes_relayer_components_extra::batch::worker::LogBatchWorker;

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
        (): HandleCosmosLogs,
    }
}

impl<'a> DelegateComponent<LogSendMessagesWithSignerAndNonce<'a, CosmosChain>>
    for CosmosLogHandlers
{
    type Delegate = HandleCosmosLogs;
}

impl<'a> DelegateComponent<TxNoResponseError<'a, CosmosChain>> for CosmosLogHandlers {
    type Delegate = HandleCosmosLogs;
}

impl<'a> DelegateComponent<LogRetryQueryTxResponse<'a, CosmosChain>> for CosmosLogHandlers {
    type Delegate = HandleCosmosLogs;
}

impl<'a> DelegateComponent<LogSkipRelayLockedPacket<'a, CosmosRelay>> for CosmosLogHandlers {
    type Delegate = HandleCosmosLogs;
}

impl<'a> DelegateComponent<LogRelayPacketAction<'a, CosmosRelay>> for CosmosLogHandlers {
    type Delegate = HandleCosmosLogs;
}

impl<'a> DelegateComponent<LogClearPacketError<'a, CosmosRelay>> for CosmosLogHandlers {
    type Delegate = HandleCosmosLogs;
}

impl<'a> DelegateComponent<LogRelayPacketStatus<'a, CosmosRelay>> for CosmosLogHandlers {
    type Delegate = HandleCosmosLogs;
}

impl<'a> DelegateComponent<LogSkipBuildUpdateClientMessage<'a, CosmosRelay, SourceTarget>>
    for CosmosLogHandlers
{
    type Delegate = HandleCosmosLogs;
}

impl<'a> DelegateComponent<LogSkipBuildUpdateClientMessage<'a, CosmosRelay, DestinationTarget>>
    for CosmosLogHandlers
{
    type Delegate = HandleCosmosLogs;
}

impl<'a> DelegateComponent<LogWaitUpdateClientHeightStatus<'a, CosmosRelay, SourceTarget>>
    for CosmosLogHandlers
{
    type Delegate = HandleCosmosLogs;
}

impl<'a> DelegateComponent<LogWaitUpdateClientHeightStatus<'a, CosmosRelay, DestinationTarget>>
    for CosmosLogHandlers
{
    type Delegate = HandleCosmosLogs;
}

impl<'a> DelegateComponent<LogBatchWorker<'a, CosmosRelay, SourceTarget>> for CosmosLogHandlers {
    type Delegate = HandleCosmosLogs;
}

impl<'a> DelegateComponent<LogBatchWorker<'a, CosmosRelay, DestinationTarget>>
    for CosmosLogHandlers
{
    type Delegate = HandleCosmosLogs;
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
