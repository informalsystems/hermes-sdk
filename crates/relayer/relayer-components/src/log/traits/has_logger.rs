use cgp_core::prelude::*;

#[derive_component(LoggerTypeComponent, ProvideLoggerType<Context>)]
pub trait HasLoggerType: Async {
    type Logger: Async;
}

#[derive_component(LoggerGetterComponent, LoggerGetter<Context>)]
pub trait HasLogger: HasLoggerType {
    fn logger(&self) -> &Self::Logger;
}

#[derive_component(GlobalLoggerGetterComponent, GlobalLoggerGetter<Context>)]
pub trait HasGlobalLogger: HasLoggerType {
    fn global_logger() -> &'static Self::Logger;
}
