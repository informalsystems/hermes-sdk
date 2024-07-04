use std::path::{Path, PathBuf};

use hermes_cli_framework::application::Application;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_runtime::types::runtime::HermesRuntime;

use crate::commands::HermesCommand;
use crate::config::HermesConfig;
use crate::Result;

#[derive(clap::Parser)]
pub struct HermesCli {
    #[clap(short = 'c', long = "config", default_value = "config.toml")]
    pub config_path: PathBuf,

    #[clap(long)]
    pub json: bool,

    #[clap(subcommand)]
    pub command: HermesCommand,
}

impl Application for HermesCli {
    type Config = HermesConfig;
    type Build = CosmosBuilder;
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

    async fn run(&self, runtime: HermesRuntime, config: Self::Config) -> Result<Output> {
        let builder = CosmosBuilder::new(
            config.config,
            runtime,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        self.command.run(&builder).await
    }
}
