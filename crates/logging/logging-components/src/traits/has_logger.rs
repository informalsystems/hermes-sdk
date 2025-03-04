use cgp::prelude::*;

#[cgp_type {
    provider: LoggerTypeProvider,
}]
pub trait HasLoggerType: Async {
    type Logger: Async;
}

#[cgp_component {
    provider: LoggerGetter,
}]
pub trait HasLogger: HasLoggerType {
    fn logger(&self) -> &Self::Logger;
}

#[cgp_component {
    provider: GlobalLoggerGetter,
}]
pub trait HasGlobalLogger: HasLoggerType {
    fn global_logger() -> &'static Self::Logger;
}
