mod log;

mod boot;
use std::path::Path;

pub use boot::boot;
use cgp::prelude::*;
use hermes_runtime::types::runtime::HermesRuntime;

use crate::command::CommandRunner;
use crate::config::Config;
use crate::output::Output;
use crate::Result;

#[async_trait]
pub trait Application: Sized {
    type Config: Config;

    type App;

    type Command: CommandRunner<Self::App>;

    fn parse_from_env() -> Self;

    fn config_path(&self) -> &Path;

    fn json_output(&self) -> bool;

    async fn run(&self, runtime: HermesRuntime) -> Result<Output>;
}
