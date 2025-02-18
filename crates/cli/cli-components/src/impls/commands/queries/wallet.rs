use cgp::prelude::*;

use crate::impls::commands::queries::balance::QueryBalanceArgs;
use crate::traits::command::{CanRunCommand, CommandRunner, CommandRunnerComponent};

pub struct RunQueryWalletSubCommand;

#[derive(Debug, clap::Subcommand)]
pub enum QueryWalletSubCommand {
    /// Query wallet balance
    Balance(QueryBalanceArgs),
}

#[cgp_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, QueryWalletSubCommand> for RunQueryWalletSubCommand
where
    App: CanRunCommand<QueryBalanceArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &QueryWalletSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            QueryWalletSubCommand::Balance(args) => app.run_command(args).await,
        }
    }
}
