use hermes_cli_framework::command::Command;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::application::HermesCli;
use crate::Result;

pub mod client;
pub mod start;

#[derive(Debug, clap::Parser)]
pub enum HermesCommand {
    /// Start the Hermes relayer
    Start(start::StartCommand),

    /// Work with clients
    #[clap(subcommand)]
    Client(client::ClientCommands),
}

impl Command<HermesCli> for HermesCommand {
    async fn run(&self, builder: CosmosBuilder) -> Result<()> {
        match self {
            Self::Start(cmd) => cmd.run(builder).await,
            Self::Client(cmd) => cmd.run(builder).await,
        }
    }
}
