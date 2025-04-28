mod add;
pub use add::KeysAddCmd;

mod list;
use hermes_cli_components::traits::CommandRunnerComponent;
pub use list::KeysListCmd;

mod delete;
pub use delete::KeysDeleteCmd;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_prelude::*;

use crate::contexts::HermesApp;
use crate::Result;

/// `keys` subcommand
#[derive(Debug, clap::Parser)]
pub enum KeysCmd {
    /// Add a key to a chain from its keyring file or restore a key using its mnemonic
    Add(KeysAddCmd),

    /// List the private key file that was added to a chain
    List(KeysListCmd),

    /// Delete key(s) from a configured chain
    Delete(KeysDeleteCmd),
}

#[cgp_provider(CommandRunnerComponent)]
impl CommandRunner<HermesApp> for KeysCmd {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Add(cmd) => cmd.run(app).await,
            Self::List(cmd) => cmd.run(app).await,
            Self::Delete(cmd) => cmd.run(app).await,
        }
    }
}
