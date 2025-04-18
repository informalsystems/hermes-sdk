use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;

#[cgp_getter {
    provider: CosmosBuilderGetter,
}]
pub trait HasCosmosBuilder {
    fn cosmos_builder(&self) -> &CosmosBuilder;
}

#[cgp_provider(CosmosBuilderGetterComponent)]
impl<Bootstrap> CosmosBuilderGetter<Bootstrap> for UseContext
where
    Bootstrap: Async + HasField<symbol!("cosmos_builder"), Value = CosmosBuilder>,
{
    fn cosmos_builder(bootstrap: &Bootstrap) -> &CosmosBuilder {
        bootstrap.get_field(PhantomData)
    }
}
