use hermes_cosmos_client_components::types::payloads::client::{
    CosmosCreateClientPayload, CosmosUpdateClientPayload,
};
use hermes_cosmos_client_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayload;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;
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

impl<Counterparty> HasClientStateType<Counterparty> for CosmosChain {
    type ClientState = TendermintClientState;
}

impl<Counterparty> HasConsensusStateType<Counterparty> for CosmosChain {
    type ConsensusState = TendermintConsensusState;
}

impl<Counterparty> HasCreateClientPayload<Counterparty> for CosmosChain {
    type CreateClientPayload = CosmosCreateClientPayload;
}

impl<Counterparty> HasUpdateClientPayload<Counterparty> for CosmosChain {
    type UpdateClientPayload = CosmosUpdateClientPayload;
}
