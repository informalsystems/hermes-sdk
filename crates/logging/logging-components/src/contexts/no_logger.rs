use cgp::prelude::*;

use crate::impls::global::GetGlobalLogger;
use crate::impls::ignore::IgnoreLog;
use crate::traits::has_logger::{
    GlobalLoggerGetter, GlobalLoggerGetterComponent, HasLoggerType, LoggerGetterComponent,
    LoggerTypeProvider, LoggerTypeProviderComponent,
};
use crate::traits::logger::{CanLog, LoggerComponent};

pub struct ProvideNoLogger;

#[cgp_context(NoLoggerComponents)]
pub struct NoLogger;

pub trait CanUseNoLogger<Details>: CanLog<Details>
where
    Details: Send + Sync,
{
}

impl<Details> CanUseNoLogger<Details> for NoLogger where Details: Send + Sync {}

#[cgp_provider(LoggerTypeProviderComponent)]
impl<Context> LoggerTypeProvider<Context> for ProvideNoLogger
where
    Context: Async,
{
    type Logger = NoLogger;
}

#[cgp_provider(GlobalLoggerGetterComponent)]
impl<Context> GlobalLoggerGetter<Context> for ProvideNoLogger
where
    Context: HasLoggerType<Logger = NoLogger>,
{
    fn global_logger() -> &'static NoLogger {
        &NoLogger
    }
}

delegate_components! {
    NoLoggerComponents {
        LoggerComponent: IgnoreLog,
    }
}

delegate_components! {
    ProvideNoLogger {
        LoggerGetterComponent: GetGlobalLogger,
    }
}
