use hermes_cli_components::impls::commands::channel::create::CreateChannelArgs;
use hermes_cli_components::traits::command::CanRunCommand;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::contexts::app::HermesApp;
use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ChannelCommands {
    /// Create a new channel
    Create(CreateChannelArgs),
}

impl CommandRunner<HermesApp> for ChannelCommands {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Create(cmd) => app.run_command(cmd).await,
        }
    }
}
