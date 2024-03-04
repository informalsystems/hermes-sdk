#![recursion_limit = "256"]

use hermes_cosmos_relayer::types::error::Error;

pub mod application;
pub mod command;
pub mod config;
pub mod output;

pub type Result<T> = core::result::Result<T, Error>;
