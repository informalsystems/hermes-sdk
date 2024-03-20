use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::logger::HandleCosmosLogs;
use hermes_relayer_components::log::impls::delegate::DelegateLogger;
use hermes_relayer_components::log::impls::global::GetGlobalLogger;
use hermes_relayer_components::log::traits::has_logger::{
    GlobalLoggerGetter, HasLoggerType, LoggerGetterComponent, ProvideLoggerType,
};
use hermes_relayer_components::log::traits::logger::{CanLog, LoggerComponent};
use hermes_relayer_components::transaction::impls::poll_tx_response::TxNoResponseError;

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
    type Delegate = HandleCosmosLogs;
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
