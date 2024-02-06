use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_relayer_components::logger::traits::log::CanLog;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct StartCommand;

impl StartCommand {
    pub async fn run(&self, builder: CosmosBuilder) -> Result<()> {
        builder.log_info("Starting relayer...");

        Ok(())
    }
}
