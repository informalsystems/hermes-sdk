use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

pub mod channel;
pub mod clear;
pub mod client;
pub mod connection;
pub mod keys;
pub mod query;
pub mod start;

#[derive(Debug, clap::Parser)]
pub enum HermesCommand {
    /// Start the Hermes relayer
    Start(start::Start),

    /// Work with clients
    #[clap(subcommand)]
    Client(client::ClientCommands),

    /// Work with connections
    #[clap(subcommand)]
    Connection(connection::ConnectionCommands),

    /// Work with channels
    #[clap(subcommand)]
    Channel(channel::ChannelCommands),

    /// Query information about IBC objects
    #[clap(subcommand)]
    Query(query::QueryCommands),

    /// Clear subcommands
    #[clap(subcommand)]
    Clear(clear::ClearCommands),

    /// Manage keys in the relayer for each chain
    #[clap(subcommand)]
    Keys(keys::KeysCmd),
}

impl CommandRunner<CosmosBuilder> for HermesCommand {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        match self {
            Self::Start(cmd) => cmd.run(builder).await,
            Self::Client(cmd) => cmd.run(builder).await,
            Self::Connection(cmd) => cmd.run(builder).await,
            Self::Channel(cmd) => cmd.run(builder).await,
            Self::Query(cmd) => cmd.run(builder).await,
            Self::Clear(cmd) => cmd.run(builder).await,
            Self::Keys(cmd) => cmd.run(builder).await,
        }
    }
}
