use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;

#[derive_component(CosmosBuilderGetterComponent, CosmosBuilderGetter<Bootstrap>)]
pub trait HasCosmosBuilder: Async {
    fn cosmos_builder(&self) -> &CosmosBuilder;
}
