use hermes_cli_components::traits::{CanRunCommand, CommandRunnerComponent};
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_prelude::*;

use crate::contexts::HermesApp;
use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ChannelSubCommands {
    /// Create a new channel
    Create(CreateChannelArgs),
}

#[derive(Debug, clap::Parser, HasField)]
pub struct CreateChannelArgs {
    #[clap(
        long = "target-chain-id",
        required = true,
        value_name = "TARGET_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    target_chain_id: String,

    #[clap(
        long = "target-client-id",
        required = true,
        value_name = "TARGET_CLIENT_ID",
        help_heading = "REQUIRED"
    )]
    target_client_id: String,

    #[clap(
        long = "target-connection-id",
        required = true,
        value_name = "TARGET_CONNECTION_ID",
        help_heading = "REQUIRED"
    )]
    target_connection_id: String,

    #[clap(long = "target-port-id", value_name = "TARGET_PORT_ID")]
    target_port_id: String,

    #[clap(
        long = "counterparty-chain-id",
        required = true,
        value_name = "COUNTERPARTY_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    counterparty_chain_id: String,

    #[clap(
        long = "counterparty-client-id",
        required = true,
        value_name = "COUNTERPARTY_CLIENT_ID",
        help_heading = "REQUIRED"
    )]
    counterparty_client_id: String,

    #[clap(long = "counterparty-port-id", value_name = "COUNTERPARTY_PORT_ID")]
    counterparty_port_id: String,

    #[clap(long = "ordering", value_name = "ORDERING")]
    ordering: String,

    #[clap(long = "version", value_name = "VERSION")]
    version: String,
}

#[cgp_provider(CommandRunnerComponent)]
impl CommandRunner<HermesApp> for ChannelSubCommands {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::Create(cmd) => app.run_command(cmd).await,
        }
    }
}
