#![recursion_limit = "256"]

pub mod application;
pub mod command;
pub mod config;

pub type Result<T> = oneline_eyre::Result<T>;
