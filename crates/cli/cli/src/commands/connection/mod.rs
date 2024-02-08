mod create;
pub use create::ConnectionCreate;

use hermes_cli_framework::command::Runnable;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ConnectionCommands {
    /// Create a new connection
    Create(ConnectionCreate),
}

impl ConnectionCommands {
    pub async fn run(&self, builder: CosmosBuilder) -> Result<()> {
        match self {
            Self::Create(cmd) => cmd.run(builder).await,
        }
    }
}
