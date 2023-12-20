use cgp_core::Async;
use hermes_relayer_components::logger::traits::has_logger::{
    HasLoggerType, LoggerGetter, LoggerTypeProvider,
};

use crate::types::log::logger::TracingLogger;

pub struct ProvideTracingLogger;

impl<Context> LoggerTypeProvider<Context> for ProvideTracingLogger
where
    Context: Async,
{
    type Logger = TracingLogger;
}

impl<Context> LoggerGetter<Context> for ProvideTracingLogger
where
    Context: HasLoggerType<Logger = TracingLogger>,
{
    fn logger(_context: &Context) -> &TracingLogger {
        &TracingLogger
    }
}
