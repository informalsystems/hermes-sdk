pub mod create;
pub use create::ChannelCreate;

use hermes_cli_framework::command::Runnable;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ChannelCommands {
    /// Create a new channel
    Create(ChannelCreate),
}

impl ChannelCommands {
    pub async fn run(&self, builder: CosmosBuilder) -> Result<()> {
        match self {
            Self::Create(cmd) => cmd.run(builder).await,
        }
    }
}
