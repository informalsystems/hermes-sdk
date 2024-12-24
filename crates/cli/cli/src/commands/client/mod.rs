use hermes_cli_components::impls::commands::client::update::UpdateClientArgs;
use hermes_cli_components::traits::command::CanRunCommand;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::commands::client::create::CreateClientArgs;
use crate::contexts::app::HermesApp;
use crate::Result;

pub mod create;

#[derive(Debug, clap::Subcommand)]
pub enum ClientCommands {
    /// Create a new client
    Create(CreateClientArgs),

    /// Update a client
    Update(UpdateClientArgs),
}

impl CommandRunner<HermesApp> for ClientCommands {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Create(cmd) => app.run_command(cmd).await,
            Self::Update(cmd) => app.run_command(cmd).await,
        }
    }
}
