mod log;

mod boot;
pub use boot::boot;

use std::path::Path;

use cgp_core::async_trait;
use hermes_relayer_runtime::types::runtime::HermesRuntime;

use crate::command::Runnable;
use crate::config::Config;
use crate::output::Output;
use crate::Result;

#[async_trait]
pub trait Application: Sized {
    type Config: Config;
    type Command: Runnable;

    fn parse_from_env() -> Self;

    fn config_path(&self) -> &Path;

    fn json_output(&self) -> bool;

    async fn run(&self, runtime: HermesRuntime, config: Self::Config) -> Result<Output>;
}
