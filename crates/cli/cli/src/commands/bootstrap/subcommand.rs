use cgp::prelude::*;
use hermes_cli_components::traits::{CanRunCommand, CommandRunner, CommandRunnerComponent};

use crate::commands::BootstrapCosmosChainArgs;

#[derive(Debug, clap::Subcommand)]
pub enum BootstrapSubCommand {
    Chain(BootstrapCosmosChainArgs),
}

pub struct RunBootstrapSubCommand;

#[cgp_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, BootstrapSubCommand> for RunBootstrapSubCommand
where
    App: CanRunCommand<BootstrapCosmosChainArgs>,
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
