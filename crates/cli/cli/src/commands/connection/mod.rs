use cgp::prelude::*;
use hermes_cli_components::impls::commands::connection::create::CreateConnectionArgs;
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunnerComponent};
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::contexts::app::HermesApp;
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
