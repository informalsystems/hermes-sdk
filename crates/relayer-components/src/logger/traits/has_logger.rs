use cgp_core::prelude::*;

use crate::logger::traits::logger::BaseLogger;

#[derive_component(LoggerTypeComponent, LoggerTypeProvider<Context>)]
pub trait HasLoggerType: Async {
    type Logger: BaseLogger;
}

#[derive_component(LoggerFieldComponent, LoggerGetter<Context>)]
pub trait HasLogger: HasLoggerType {
    fn logger(&self) -> &Self::Logger;
}
