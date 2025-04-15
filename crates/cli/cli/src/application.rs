use std::path::{Path, PathBuf};

use hermes_cli_framework::application::Application;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_core::runtime::types::runtime::HermesRuntime;

use crate::commands::HermesCommand;
use crate::config::HermesConfig;
use crate::contexts::HermesApp;
use crate::Result;

#[derive(clap::Parser)]
pub struct HermesCli {
    #[clap(short = 'c', long = "config")]
    pub config_path: PathBuf,

    #[clap(long)]
    pub json: bool,

    #[clap(subcommand)]
    pub command: HermesCommand,
}

impl Application for HermesCli {
    type Config = HermesConfig;

    type App = HermesApp;

    type Command = HermesCommand;

    fn config_path(&self) -> &Path {
        &self.config_path
    }

    fn json_output(&self) -> bool {
        self.json
    }

    fn parse_from_env() -> Self {
        clap::Parser::parse()
    }

    async fn run(&self, runtime: HermesRuntime) -> Result<Output> {
        let app = HermesApp {
            runtime,
            config_path: self.config_path.clone(),
        };

        self.command.run(&app).await
    }
}
