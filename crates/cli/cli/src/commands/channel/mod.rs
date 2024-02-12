pub mod create;
pub use create::ChannelCreate;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ChannelCommands {
    /// Create a new channel
    Create(ChannelCreate),
}

impl CommandRunner for ChannelCommands {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        match self {
            Self::Create(cmd) => cmd.run(builder).await,
        }
    }
}
