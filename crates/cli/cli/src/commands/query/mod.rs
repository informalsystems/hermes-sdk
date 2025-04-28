use hermes_cli_components::impls::{
    QueryChainSubCommand, QueryClientSubCommand, QueryClientsArgs, QueryConnectionSubCommand,
    QueryWalletSubCommand,
};
use hermes_cli_components::traits::{CanRunCommand, CommandRunnerComponent};
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_prelude::*;

mod connections;
pub use connections::QueryConnections;

mod channel;
pub use channel::QueryChannel;

mod channels;
pub use channels::QueryChannels;

use crate::contexts::HermesApp;
use crate::Result;

/// All subcommands for querying IBC-related objects and data.
#[derive(Debug, clap::Subcommand)]
pub enum QueryCommands {
    /// Query all clients
    Clients(QueryClientsArgs),

    /// Query all connections
    Connections(QueryConnections),

    /// Query all channels
    Channels(QueryChannels),

    /// Query information about wallet balance
    #[clap(subcommand)]
    Wallet(QueryWalletSubCommand),

    /// Query information about chains
    #[clap(subcommand)]
    Chain(QueryChainSubCommand),

    /// Query information about IBC clients
    #[clap(subcommand)]
    Client(QueryClientSubCommand),

    /// Query connection information
    #[clap(subcommand)]
    Connection(QueryConnectionSubCommand),

    /// Query channel information
    #[clap(subcommand)]
    Channel(QueryChannel),
}

#[cgp_provider(CommandRunnerComponent)]
impl CommandRunner<HermesApp> for QueryCommands {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Wallet(cmd) => app.run_command(cmd).await,
            Self::Chain(cmd) => app.run_command(cmd).await,
            Self::Client(cmd) => app.run_command(cmd).await,
            Self::Clients(cmd) => app.run_command(cmd).await,
            Self::Connection(cmd) => app.run_command(cmd).await,
            Self::Connections(cmd) => cmd.run(app).await,
            Self::Channels(cmd) => cmd.run(app).await,
            Self::Channel(cmd) => cmd.run(app).await,
        }
    }
}
