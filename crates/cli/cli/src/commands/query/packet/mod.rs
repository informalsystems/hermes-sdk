mod pending;

mod commitments;
mod pending_acks;
mod util;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
pub use pending::QueryPendingPackets;

use crate::commands::query::packet::pending_acks::QueryPendingAcks;
use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum PacketCommands {
    /// Query packet commitments
    Commitments(commitments::QueryPacketCommitments),

    /// Output a summary of pending packets in both directions of a channel
    Pending(QueryPendingPackets),

    /// Query packet acknowledgment
    PendingAcks(QueryPendingAcks),
}

impl CommandRunner<CosmosBuilder> for PacketCommands {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        match self {
            Self::Commitments(cmd) => cmd.run(builder).await,
            Self::Pending(cmd) => cmd.run(builder).await,
            Self::PendingAcks(cmd) => cmd.run(builder).await,
        }
    }
}
