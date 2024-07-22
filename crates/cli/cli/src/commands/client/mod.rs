use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::contexts::app::HermesApp;
use crate::Result;

mod create;
pub use create::ClientCreate;

mod update;
pub use update::ClientUpdate;

#[derive(Debug, clap::Subcommand)]
pub enum ClientCommands {
    /// Create a new client
    Create(ClientCreate),

    /// Update a client
    Update(ClientUpdate),
}

impl CommandRunner<HermesApp> for ClientCommands {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Create(cmd) => cmd.run(app).await,
            Self::Update(cmd) => cmd.run(app).await,
        }
    }
}
