use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;

#[derive_component(CosmosBuilderGetterComponent, CosmosBuilderGetter<Bootstrap>)]
pub trait HasCosmosBuilder: Async {
    fn cosmos_builder(&self) -> &CosmosBuilder;
}

impl<Bootstrap> CosmosBuilderGetter<Bootstrap> for UseContext
where
    Bootstrap: Async + HasField<symbol!("cosmos_builder"), Field: AsRef<CosmosBuilder>>,
{
    fn cosmos_builder(bootstrap: &Bootstrap) -> &CosmosBuilder {
        bootstrap.get_field(PhantomData).as_ref()
    }
}
