mod packets;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
pub use packets::PacketsClear;

use crate::contexts::HermesApp;
use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ClearCommands {
    /// Clear pending packets
    Packets(PacketsClear),
}

impl CommandRunner<HermesApp> for ClearCommands {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Packets(cmd) => cmd.run(app).await,
        }
    }
}
