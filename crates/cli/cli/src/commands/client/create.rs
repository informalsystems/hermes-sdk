use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct Create {}

impl Create {
    pub async fn run(&self, _builder: CosmosBuilder) -> Result<()> {
        todo!()
    }
}
