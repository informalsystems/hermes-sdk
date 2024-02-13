mod client;
pub use client::ClientCommands;

mod clients;
pub use clients::QueryClients;

mod connection;
pub use connection::QueryConnection;

mod connections;
pub use connections::QueryConnections;

mod channels;
pub use channels::QueryChannels;

mod packet;
pub use packet::PacketCommands;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

/// All subcommands for querying IBC-related objects and data.
#[derive(Debug, clap::Subcommand)]
pub enum QueryCommands {
    /// Query information about an IBC client
    #[clap(subcommand)]
    Client(ClientCommands),

    /// Query information about IBC clients
    Clients(QueryClients),

    /// Query all connections
    Connections(QueryConnections),

    /// Query all channels
    Channels(QueryChannels),

    /// Query connection information
    #[clap(subcommand)]
    Connection(QueryConnection),

    /// Query information about IBC packets
    #[clap(subcommand)]
    Packet(PacketCommands),
}

impl CommandRunner<CosmosBuilder> for QueryCommands {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        match self {
            Self::Client(cmd) => cmd.run(builder).await,
            Self::Clients(cmd) => cmd.run(builder).await,
            Self::Connections(cmd) => cmd.run(builder).await,
            Self::Channels(cmd) => cmd.run(builder).await,
            Self::Connection(cmd) => cmd.run(builder).await,
            Self::Packet(cmd) => cmd.run(builder).await,
        }
    }
}
