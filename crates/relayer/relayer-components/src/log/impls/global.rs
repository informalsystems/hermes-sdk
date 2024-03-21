use crate::log::traits::has_logger::{HasGlobalLogger, LoggerGetter};

pub struct GetGlobalLogger;

impl<Context> LoggerGetter<Context> for GetGlobalLogger
where
    Context: HasGlobalLogger,
{
    fn logger(_context: &Context) -> &Context::Logger {
        Context::global_logger()
    }
}
