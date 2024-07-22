mod pending;

mod commitments;
mod pending_acks;
mod util;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
pub use pending::QueryPendingPackets;

use crate::commands::query::packet::pending_acks::QueryPendingAcks;
use crate::contexts::app::HermesApp;
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

impl CommandRunner<HermesApp> for PacketCommands {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Commitments(cmd) => cmd.run(app).await,
            Self::Pending(cmd) => cmd.run(app).await,
            Self::PendingAcks(cmd) => cmd.run(app).await,
        }
    }
}
