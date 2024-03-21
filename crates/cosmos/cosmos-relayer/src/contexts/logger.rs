use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::logger::HandleCosmosLogs;
use hermes_relayer_components::log::impls::delegate::DelegateLogger;
use hermes_relayer_components::log::impls::global::GetGlobalLogger;
use hermes_relayer_components::log::traits::has_logger::{
    GlobalLoggerGetter, HasLoggerType, LoggerGetterComponent, ProvideLoggerType,
};
use hermes_relayer_components::log::traits::logger::{CanLog, LoggerComponent};
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use hermes_relayer_components::transaction::impls::poll_tx_response::{
    LogRetryQueryTxResponse, TxNoResponseError,
};

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
