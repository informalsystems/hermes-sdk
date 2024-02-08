use cgp_core::async_trait;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[async_trait]
pub trait Runnable {
    async fn run(&self, builder: CosmosBuilder) -> Result<()>;
}
