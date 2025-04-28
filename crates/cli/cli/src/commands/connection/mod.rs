use hermes_cli_components::impls::CreateConnectionArgs;
use hermes_cli_components::traits::{CanRunCommand, CommandRunnerComponent};
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_prelude::*;

use crate::contexts::HermesApp;
use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ConnectionCommands {
    /// Create a new connection
    Create(CreateConnectionArgs),
}

#[cgp_provider(CommandRunnerComponent)]
impl CommandRunner<HermesApp> for ConnectionCommands {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Create(cmd) => app.run_command(cmd).await,
        }
    }
}
