mod client;
pub use client::ClientCommands;

mod connection;
pub use connection::QueryConnection;

mod connections;
pub use connections::QueryConnections;

mod packet;
pub use packet::PacketCommands;

use hermes_cli_framework::command::Runnable;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum QueryCommands {
    /// Query information about IBC clients
    #[clap(subcommand)]
    Client(ClientCommands),

    /// Query all connections
    Connections(QueryConnections),

    /// Query connection information
    #[clap(subcommand)]
    Connection(QueryConnection),

    /// Query information about IBC packets
    #[clap(subcommand)]
    Packet(PacketCommands),
}

impl Runnable for QueryCommands {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        match self {
            Self::Client(cmd) => cmd.run(builder).await,
            Self::Connections(cmd) => cmd.run(builder).await,
<<<<<<< HEAD
            Self::Packet(cmd) => cmd.run(builder).await,
=======
            Self::Connection(cmd) => cmd.run(builder).await,
>>>>>>> main
        }
    }
}
