use cgp::prelude::*;
use hermes_cli_components::impls::commands::start::StartRelayerArgs;
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunnerComponent};
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::contexts::app::HermesApp;
use crate::Result;

pub mod bootstrap;
pub mod channel;
pub mod client;
pub mod connection;
pub mod query;

pub mod keys;

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
    Bootstrap(bootstrap::subcommand::BootstrapSubCommand),
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
