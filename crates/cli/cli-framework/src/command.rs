use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[allow(async_fn_in_trait)]
pub trait Runnable {
    async fn run(&self, builder: CosmosBuilder) -> Result<()>;
}
