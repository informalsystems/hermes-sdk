use core::fmt::Debug;

use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::application::Application;
use crate::Result;

#[allow(async_fn_in_trait)]
pub trait Command<A>: Debug
where
    A: Application,
{
    async fn run(&self, builder: CosmosBuilder) -> Result<()>;
}
