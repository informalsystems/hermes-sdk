use crate::logger::traits::logger::BaseLogger;
use cgp_core::traits::sync::Async;

pub trait HasLoggerType: Async {
    type Logger: BaseLogger;
}

pub trait HasLogger: HasLoggerType {
    fn logger(&self) -> &Self::Logger;
}
