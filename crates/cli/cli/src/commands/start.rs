use hermes_cli_framework::command::Runnable;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_relayer_components::logger::traits::log::CanLog;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct Start;

impl Runnable for Start {
    async fn run(&self, builder: CosmosBuilder) -> Result<()> {
        builder.log_info("Starting relayer...");

        Ok(())
    }
}
