use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;

#[cgp_component {
  name: CosmosBuilderGetterComponent,
  provider: CosmosBuilderGetter,
  context: Bootstrap,
}]
pub trait HasCosmosBuilder: Async {
    fn cosmos_builder(&self) -> &CosmosBuilder;
}

impl<Bootstrap> CosmosBuilderGetter<Bootstrap> for UseContext
where
    Bootstrap: Async + HasField<symbol!("cosmos_builder"), Value = CosmosBuilder>,
{
    fn cosmos_builder(bootstrap: &Bootstrap) -> &CosmosBuilder {
        bootstrap.get_field(PhantomData)
    }
}
