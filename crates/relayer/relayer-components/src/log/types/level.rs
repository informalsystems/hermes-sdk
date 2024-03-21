#[derive(Copy, Clone)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub struct LevelError;

pub struct LevelWarn;

pub struct LevelInfo;

pub struct LevelDebug;

pub struct LevelTrace;
