mod log;

mod boot;
pub use boot::boot;

use std::path::Path;

use hermes_relayer_runtime::types::runtime::HermesRuntime;

use crate::command::Command;
use crate::config::Config;
use crate::Result;

#[allow(async_fn_in_trait)]
pub trait Application: Sized {
    type Config: Config;
    type Command: Command<Self>;

    fn parse_from_env() -> Self;

    fn config_path(&self) -> &Path;

    async fn run(&self, runtime: HermesRuntime, config: Self::Config) -> Result<()>;
}
