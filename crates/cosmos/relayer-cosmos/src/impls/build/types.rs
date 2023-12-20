use hermes_relayer_components::build::traits::birelay::HasBiRelayType;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::builder::CosmosBuilder;
use crate::impls::build::components::CosmosBuildComponents;

impl HasBiRelayType for CosmosBuilder {
    type BiRelay = CosmosBiRelay<BaseChainHandle, BaseChainHandle>;
}

impl ProvideRuntime<CosmosBuilder> for CosmosBuildComponents {
    fn runtime(build: &CosmosBuilder) -> &TokioRuntimeContext {
        &build.runtime
    }
}
