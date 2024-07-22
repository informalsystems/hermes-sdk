pub mod create;
pub use create::ChannelCreate;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::contexts::app::HermesApp;
use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ChannelCommands {
    /// Create a new channel
    Create(ChannelCreate),
}

impl CommandRunner<HermesApp> for ChannelCommands {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Create(cmd) => cmd.run(app).await,
        }
    }
}
