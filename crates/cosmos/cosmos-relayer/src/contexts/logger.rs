use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::logger::HandleCosmosLogs;
use hermes_relayer_components::log::impls::delegate::DelegateLogger;
use hermes_relayer_components::log::impls::global::GetGlobalLogger;
use hermes_relayer_components::log::traits::has_logger::{
    GlobalLoggerGetter, HasLoggerType, LoggerGetter, LoggerGetterComponent, ProvideLoggerType,
};
use hermes_relayer_components::log::traits::logger::LoggerComponent;
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;

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

impl<'a> DelegateComponent<LogSendMessagesWithSignerAndNonce<'a, CosmosChain>>
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
