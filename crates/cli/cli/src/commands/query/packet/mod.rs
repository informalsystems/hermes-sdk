mod pending;
mod util;
use hermes_cli_framework::output::Output;
pub use pending::QueryPendingPackets;

use hermes_cli_framework::command::Runnable;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum PacketCommands {
    /// Output a summary of pending packets in both directions of a channel
    Pending(QueryPendingPackets),
}

impl Runnable for PacketCommands {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        match self {
            Self::Pending(cmd) => cmd.run(builder).await,
        }
    }
}
