mod client;
pub use client::ClientCommands;

mod connection;
pub use connection::QueryConnection;

mod clients;
pub use clients::QueryClients;

mod connections;
pub use connections::QueryConnections;

mod channel;
pub use channel::QueryChannel;

mod channels;
pub use channels::QueryChannels;

mod packet;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
pub use packet::PacketCommands;

use crate::Result;

/// All subcommands for querying IBC-related objects and data.
#[derive(Debug, clap::Subcommand)]
pub enum QueryCommands {
    /// Query all clients
    Clients(QueryClients),

    /// Query all connections
    Connections(QueryConnections),

    /// Query all channels
    Channels(QueryChannels),

    /// Query information about IBC clients
    #[clap(subcommand)]
    Client(ClientCommands),

    /// Query connection information
    #[clap(subcommand)]
    Connection(QueryConnection),

    /// Query channel information
    #[clap(subcommand)]
    Channel(QueryChannel),

    /// Query information about IBC packets
    #[clap(subcommand)]
    Packet(PacketCommands),
}

impl CommandRunner<CosmosBuilder> for QueryCommands {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        match self {
            Self::Client(cmd) => cmd.run(builder).await,
            Self::Clients(cmd) => cmd.run(builder).await,
            Self::Connection(cmd) => cmd.run(builder).await,
            Self::Connections(cmd) => cmd.run(builder).await,
            Self::Channels(cmd) => cmd.run(builder).await,
            Self::Channel(cmd) => cmd.run(builder).await,
            Self::Packet(cmd) => cmd.run(builder).await,
        }
    }
}
