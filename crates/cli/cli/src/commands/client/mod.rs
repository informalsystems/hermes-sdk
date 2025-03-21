use cgp::prelude::*;
use hermes_cli_components::impls::commands::client::update::UpdateClientArgs;
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunnerComponent};
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::commands::client::create::CreateCosmosClientArgs;
use crate::contexts::app::HermesApp;
use crate::Result;

pub mod create;

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
