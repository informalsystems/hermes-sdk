use hermes_error::types::Error;

pub mod application;
pub mod command;
pub mod config;
pub mod output;

pub type Result<T> = core::result::Result<T, Error>;
