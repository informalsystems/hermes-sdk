use cgp::core::component::UseContext;
use hermes_cosmos_relayer::contexts::CosmosBuilder;
use hermes_prelude::*;

#[cgp_getter {
    provider: CosmosBuilderGetter,
}]
pub trait HasCosmosBuilder {
    fn cosmos_builder(&self) -> &CosmosBuilder;
}
