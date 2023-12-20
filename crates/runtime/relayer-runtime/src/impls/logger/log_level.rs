use ibc_relayer_components::logger::traits::level::{
    HasLogLevel, LevelDebug, LevelError, LevelInfo, LevelTrace, LevelWarn,
};

use crate::types::log::level::LogLevel;
use crate::types::log::logger::TracingLogger;

impl HasLogLevel<LevelTrace> for TracingLogger {
    const LEVEL: LogLevel = LogLevel::Trace;
}

impl HasLogLevel<LevelDebug> for TracingLogger {
    const LEVEL: LogLevel = LogLevel::Debug;
}

impl HasLogLevel<LevelInfo> for TracingLogger {
    const LEVEL: LogLevel = LogLevel::Info;
}

impl HasLogLevel<LevelWarn> for TracingLogger {
    const LEVEL: LogLevel = LogLevel::Warn;
}

impl HasLogLevel<LevelError> for TracingLogger {
    const LEVEL: LogLevel = LogLevel::Error;
}
