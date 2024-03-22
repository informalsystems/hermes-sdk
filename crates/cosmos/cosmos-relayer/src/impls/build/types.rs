use hermes_relayer_components::build::traits::birelay::ProvideBiRelayType;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeGetter;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::builder::CosmosBuilder;
use crate::impls::build::components::CosmosBuildComponents;

impl ProvideBiRelayType<CosmosBuilder> for CosmosBuildComponents {
    type BiRelay = CosmosBiRelay;
}

impl RuntimeGetter<CosmosBuilder> for CosmosBuildComponents {
    fn runtime(build: &CosmosBuilder) -> &HermesRuntime {
        &build.runtime
    }
}
