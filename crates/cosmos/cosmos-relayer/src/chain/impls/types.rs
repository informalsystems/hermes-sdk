use hermes_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeGetter;

use crate::chain::components::CosmosChainComponents;
use crate::contexts::chain::CosmosChain;
use crate::types::telemetry::CosmosTelemetry;

impl RuntimeGetter<CosmosChain> for CosmosChainComponents {
    fn runtime(chain: &CosmosChain) -> &HermesRuntime {
        &chain.runtime
    }
}

impl HasTelemetry for CosmosChain {
    type Telemetry = CosmosTelemetry;

    fn telemetry(&self) -> &CosmosTelemetry {
        &self.telemetry
    }
}
