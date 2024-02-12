use cgp_core::async_trait;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::output::Output;
use crate::Result;

#[async_trait]
pub trait CommandRunner {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output>;
}
