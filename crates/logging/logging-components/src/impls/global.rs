use cgp::prelude::*;

use crate::traits::has_logger::{HasGlobalLogger, LoggerGetter, LoggerGetterComponent};

pub struct GetGlobalLogger;

#[cgp_provider(LoggerGetterComponent)]
impl<Context> LoggerGetter<Context> for GetGlobalLogger
where
    Context: HasGlobalLogger,
{
    fn logger(_context: &Context) -> &Context::Logger {
        Context::global_logger()
    }
}
