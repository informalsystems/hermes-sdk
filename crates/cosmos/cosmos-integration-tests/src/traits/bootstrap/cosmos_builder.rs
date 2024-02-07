use cgp_core::prelude::*;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

#[derive_component(CosmosBuilderGetterComponent, CosmosBuilderGetter<Bootstrap>)]
pub trait HasCosmosBuilder: Async {
    fn cosmos_builder(&self) -> &CosmosBuilder;
}
