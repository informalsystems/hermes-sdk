use hermes_cli_components::traits::command::{CanRunCommand, CommandRunner};

use crate::commands::bootstrap::chain::BootstrapChainArgs;

#[derive(Debug, clap::Subcommand)]
pub enum BootstrapSubCommand {
    /// Query the state of a client
    Chain(BootstrapChainArgs),
}

pub struct RunBootstrapSubCommand;

impl<App> CommandRunner<App, BootstrapSubCommand> for RunBootstrapSubCommand
where
    App: CanRunCommand<BootstrapChainArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &BootstrapSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            BootstrapSubCommand::Chain(args) => app.run_command(args).await,
        }
    }
}
