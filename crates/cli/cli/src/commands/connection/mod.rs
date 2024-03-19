mod create;
pub use create::ConnectionCreate;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ConnectionCommands {
    /// Create a new connection
    Create(ConnectionCreate),
}

impl CommandRunner<CosmosBuilder> for ConnectionCommands {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        match self {
            Self::Create(cmd) => cmd.run(builder).await,
        }
    }
}
