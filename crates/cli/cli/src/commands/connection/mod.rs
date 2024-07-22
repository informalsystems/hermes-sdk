mod create;
pub use create::ConnectionCreate;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::contexts::app::HermesApp;
use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ConnectionCommands {
    /// Create a new connection
    Create(ConnectionCreate),
}

impl CommandRunner<HermesApp> for ConnectionCommands {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Create(cmd) => cmd.run(app).await,
        }
    }
}
