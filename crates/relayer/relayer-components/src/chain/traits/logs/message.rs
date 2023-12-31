use crate::chain::traits::types::message::HasMessageType;
use crate::logger::traits::has_logger::HasLoggerType;
use crate::logger::traits::logger::BaseLogger;

pub trait CanLogChainMessage: HasMessageType + HasLoggerType {
    fn log_message<'a>(message: &'a Self::Message) -> <Self::Logger as BaseLogger>::LogValue<'a>;
}
