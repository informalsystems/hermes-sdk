use ibc_relayer_components::logger::traits::has_logger::{
    HasLogger, HasLoggerType, LoggerGetter, LoggerTypeProvider,
};

use crate::traits::inner::HasInner;

pub struct ForwardLogger;

impl<Context, Inner> LoggerTypeProvider<Context> for ForwardLogger
where
    Context: HasInner<Inner = Inner>,
    Inner: HasLoggerType,
{
    type Logger = Inner::Logger;
}

impl<Context, Inner> LoggerGetter<Context> for ForwardLogger
where
    Context: HasInner<Inner = Inner> + HasLoggerType<Logger = Inner::Logger>,
    Inner: HasLogger,
{
    fn logger(context: &Context) -> &<Context as HasLoggerType>::Logger {
        context.inner().logger()
    }
}
