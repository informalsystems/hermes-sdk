mod add;
pub use add::KeysAddCmd;

mod list;
pub use list::KeysListCmd;

mod delete;
pub use delete::KeysDeleteCmd;

mod balance;
pub use balance::KeysBalanceCmd;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::contexts::app::HermesApp;
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

    /// Retrieve the balance for a key from a configured chain
    Balance(KeysBalanceCmd),
}

impl CommandRunner<HermesApp> for KeysCmd {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Add(cmd) => cmd.run(app).await,
            Self::List(cmd) => cmd.run(app).await,
            Self::Delete(cmd) => cmd.run(app).await,
            Self::Balance(cmd) => cmd.run(app).await,
        }
    }
}
