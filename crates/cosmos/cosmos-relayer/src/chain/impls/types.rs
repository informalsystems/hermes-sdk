use hermes_cosmos_client_components::types::tendermint::TendermintConsensusState;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use hermes_relayer_runtime::types::runtime::HermesRuntime;

use crate::chain::components::CosmosChainComponents;
use crate::contexts::chain::CosmosChain;
use crate::types::telemetry::CosmosTelemetry;

impl ProvideRuntime<CosmosChain> for CosmosChainComponents {
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

impl<Counterparty> HasConsensusStateType<Counterparty> for CosmosChain {
    type ConsensusState = TendermintConsensusState;
}
