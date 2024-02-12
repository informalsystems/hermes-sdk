mod client;
pub use client::ClientCommands;

mod connections;
pub use connections::QueryConnections;

mod channels;
pub use channels::QueryChannels;

use hermes_cli_framework::command::Runnable;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

/// All subcommands for querying IBC-related objects and data.
#[derive(Debug, clap::Subcommand)]
pub enum QueryCommands {
    /// Query information about IBC clients
    #[clap(subcommand)]
    Client(ClientCommands),

    /// Query all connections
    Connections(QueryConnections),

    /// Query all channels
    Channels(QueryChannels),
}

impl Runnable for QueryCommands {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        match self {
            Self::Client(cmd) => cmd.run(builder).await,
            Self::Connections(cmd) => cmd.run(builder).await,
            Self::Channels(cmd) => cmd.run(builder).await,
        }
    }
}
