mod add;
pub use add::KeysAddCmd;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

/// `keys` subcommand
#[derive(Debug, clap::Parser)]
pub enum KeysCmd {
    /// Add a key to a chain from its keyring file or restore a key using its mnemonic
    Add(KeysAddCmd),
}

impl CommandRunner<CosmosBuilder> for KeysCmd {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        match self {
            Self::Add(cmd) => cmd.run(builder).await,
        }
    }
}
