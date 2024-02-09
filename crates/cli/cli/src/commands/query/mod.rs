mod connection;
mod connections;
pub use connection::QueryConnection;
pub use connections::QueryConnections;

use hermes_cli_framework::command::Runnable;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum QueryCommands {
    /// Query all connections
    Connections(QueryConnections),

    /// Query connection information
    #[clap(subcommand)]
    Connection(QueryConnection),
}

impl QueryCommands {
    pub async fn run(&self, builder: CosmosBuilder) -> Result<()> {
        match self {
            Self::Connections(cmd) => cmd.run(builder).await,
            Self::Connection(cmd) => cmd.run(builder).await,
        }
    }
}
