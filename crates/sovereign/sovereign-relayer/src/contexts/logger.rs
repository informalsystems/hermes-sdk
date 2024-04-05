use cgp_core::prelude::*;
use hermes_logging_components::impls::delegate::DelegateLogger;
use hermes_logging_components::impls::global::GetGlobalLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetter, HasLoggerType, LoggerGetterComponent, ProvideLoggerType,
};
use hermes_logging_components::traits::logger::{CanLog, LoggerComponent};
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use hermes_relayer_components::transaction::impls::poll_tx_response::{
    LogRetryQueryTxResponse, TxNoResponseError,
};
use hermes_tracing_logging_components::contexts::logger::TracingLogger;

use crate::contexts::sovereign_rollup::SovereignRollup;

pub struct SovereignLogger;

pub struct SovereignLoggerComponents;

pub struct SovereignLogHandlers;

impl HasComponents for SovereignLogger {
    type Components = SovereignLoggerComponents;
}

delegate_components! {
    SovereignLoggerComponents {
        LoggerComponent: DelegateLogger<SovereignLogHandlers>,
    }
}

pub trait CanUseSovereignLogger: for<'a> CanLog<TxNoResponseError<'a, SovereignRollup>> {}

impl CanUseSovereignLogger for SovereignLogger {}

impl<'a> DelegateComponent<TxNoResponseError<'a, SovereignRollup>> for SovereignLogHandlers {
    type Delegate = TracingLogger;
}

impl<'a> DelegateComponent<LogRetryQueryTxResponse<'a, SovereignRollup>> for SovereignLogHandlers {
    type Delegate = TracingLogger;
}

impl<'a> DelegateComponent<LogSendMessagesWithSignerAndNonce<'a, SovereignRollup>>
    for SovereignLogHandlers
{
    type Delegate = TracingLogger;
}

pub struct ProvideSovereignLogger;

delegate_components! {
    ProvideSovereignLogger {
        LoggerGetterComponent: GetGlobalLogger,
    }
}

impl<Context> ProvideLoggerType<Context> for ProvideSovereignLogger
where
    Context: Async,
{
    type Logger = SovereignLogger;
}

impl<Context> GlobalLoggerGetter<Context> for ProvideSovereignLogger
where
    Context: HasLoggerType<Logger = SovereignLogger>,
{
    fn global_logger() -> &'static SovereignLogger {
        &SovereignLogger
    }
}
