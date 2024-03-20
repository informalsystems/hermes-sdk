use cgp_core::prelude::*;

pub trait HasLoggerType: Async {
    type Logger: Async;
}

pub trait HasLogger: HasLoggerType {
    fn logger(&self) -> &Self::Logger;
}

pub trait HasGlobalLogger: HasLoggerType {
    fn global_logger() -> &'static Self::Logger;
}
