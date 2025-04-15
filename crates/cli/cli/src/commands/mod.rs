use cgp::prelude::*;
use hermes_cli_components::impls::StartRelayerArgs;
use hermes_cli_components::traits::{CanRunCommand, CommandRunnerComponent};
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::contexts::HermesApp;
use crate::Result;

mod bootstrap;
pub use bootstrap::*;

mod channel;
pub use channel::*;

mod client;
pub use client::*;

mod connection;
pub use connection::*;

mod query;
pub use query::*;

mod keys;
pub use keys::*;

#[derive(Debug, clap::Parser)]
pub enum HermesCommand {
    /// Start the Hermes relayer
    Start(StartRelayerArgs),

    /// Work with clients
    #[clap(subcommand)]
    Client(client::ClientCommands),

    /// Work with connections
    #[clap(subcommand)]
    Connection(connection::ConnectionCommands),

    /// Work with channels
    #[clap(subcommand)]
    Channel(channel::ChannelSubCommands),

    /// Query information about IBC objects
    #[clap(subcommand)]
    Query(query::QueryCommands),

    /// Manage keys in the relayer for each chain
    #[clap(subcommand)]
    Keys(keys::KeysCmd),

    #[clap(subcommand)]
    Bootstrap(BootstrapSubCommand),
}

#[cgp_provider(CommandRunnerComponent)]
impl CommandRunner<HermesApp> for HermesCommand {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Start(cmd) => app.run_command(cmd).await,
            Self::Client(cmd) => cmd.run(app).await,
            Self::Connection(cmd) => cmd.run(app).await,
            Self::Channel(cmd) => cmd.run(app).await,
            Self::Query(cmd) => cmd.run(app).await,
            Self::Keys(cmd) => cmd.run(app).await,
            Self::Bootstrap(cmd) => app.run_command(cmd).await,
        }
    }
}
