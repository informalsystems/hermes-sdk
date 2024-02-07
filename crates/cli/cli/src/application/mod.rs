use std::path::{Path, PathBuf};

use hermes_cli_framework::application::Application;
use hermes_cli_framework::command::Runnable;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_relayer_runtime::types::runtime::HermesRuntime;

use crate::commands::HermesCommand;
use crate::config::HermesConfig;
use crate::Result;

#[derive(clap::Parser)]
pub struct HermesCli {
    #[clap(short = 'c', long = "config", default_value = "config.toml")]
    pub config_path: PathBuf,

    #[clap(subcommand)]
    pub command: HermesCommand,
}

impl Application for HermesCli {
    type Config = HermesConfig;
    type Command = HermesCommand;

    fn config_path(&self) -> &Path {
        &self.config_path
    }

    fn parse_from_env() -> Self {
        clap::Parser::parse()
    }

    async fn run(&self, runtime: HermesRuntime, config: Self::Config) -> Result<()> {
        let builder = CosmosBuilder::new(
            config.config,
            runtime,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        self.command.run(builder).await
    }
}
