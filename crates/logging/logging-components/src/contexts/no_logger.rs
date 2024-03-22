use cgp_core::prelude::*;

use crate::impls::global::GetGlobalLogger;
use crate::impls::ignore::IgnoreLog;
use crate::traits::has_logger::{
    GlobalLoggerGetter, HasLoggerType, LoggerGetterComponent, ProvideLoggerType,
};
use crate::traits::logger::{CanLog, LoggerComponent};

pub struct ProvideNoLogger;

pub struct NoLogger;

pub struct NoLoggerComponents;

pub trait CanUseNoLogger<Details>: CanLog<Details>
where
    Details: Send + Sync,
{
}

impl<Details> CanUseNoLogger<Details> for NoLogger where Details: Send + Sync {}

impl<Context> ProvideLoggerType<Context> for ProvideNoLogger
where
    Context: Async,
{
    type Logger = NoLogger;
}

impl<Context> GlobalLoggerGetter<Context> for ProvideNoLogger
where
    Context: HasLoggerType<Logger = NoLogger>,
{
    fn global_logger() -> &'static NoLogger {
        &NoLogger
    }
}

impl HasComponents for NoLogger {
    type Components = NoLoggerComponents;
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
