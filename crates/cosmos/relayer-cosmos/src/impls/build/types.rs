use hermes_relayer_components::build::traits::birelay::HasBiRelayType;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer::chain::handle::BaseChainHandle;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::builder::CosmosBuilder;
use crate::impls::build::components::CosmosBuildComponents;

impl HasBiRelayType for CosmosBuilder {
    type BiRelay = CosmosBiRelay<BaseChainHandle, BaseChainHandle>;
}

impl ProvideRuntime<CosmosBuilder> for CosmosBuildComponents {
    fn runtime(build: &CosmosBuilder) -> &HermesRuntime {
        &build.runtime
    }
}
