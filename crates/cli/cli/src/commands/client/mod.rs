use cgp::prelude::*;
use hermes_cli_components::impls::UpdateClientArgs;
use hermes_cli_components::traits::{CanRunCommand, CommandRunnerComponent};
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::contexts::HermesApp;
use crate::Result;

mod create;
pub use create::*;

#[derive(Debug, clap::Subcommand)]
pub enum ClientCommands {
    /// Create a new client
    Create(CreateCosmosClientArgs),

    /// Update a client
    Update(UpdateClientArgs),
}

#[cgp_provider(CommandRunnerComponent)]
impl CommandRunner<HermesApp> for ClientCommands {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Create(cmd) => app.run_command(cmd).await,
            Self::Update(cmd) => app.run_command(cmd).await,
        }
    }
}
