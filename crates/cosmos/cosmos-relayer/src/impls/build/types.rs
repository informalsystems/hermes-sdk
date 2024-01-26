use hermes_relayer_components::build::traits::birelay::ProvideBiRelayType;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_runtime::types::runtime::HermesRuntime;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::builder::CosmosBuilder;
use crate::impls::build::components::CosmosBuildComponents;

impl ProvideBiRelayType<CosmosBuilder> for CosmosBuildComponents {
    type BiRelay = CosmosBiRelay;
}

impl ProvideRuntime<CosmosBuilder> for CosmosBuildComponents {
    fn runtime(build: &CosmosBuilder) -> &HermesRuntime {
        &build.runtime
    }
}
