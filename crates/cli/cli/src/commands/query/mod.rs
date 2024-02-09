mod connections;
pub use connections::QueryConnections;

use hermes_cli_framework::command::Runnable;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum QueryCommands {
    /// Create a new channel
    Connections(QueryConnections),
}

impl QueryCommands {
    pub async fn run(&self, builder: CosmosBuilder) -> Result<()> {
        match self {
            Self::Connections(cmd) => cmd.run(builder).await,
        }
    }
}
