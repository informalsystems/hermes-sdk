use hermes_relayer_components::birelay::traits::two_way::{
    HasTwoChainTypes, HasTwoWayRelay, HasTwoWayRelayTypes,
};
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_runtime::types::runtime::HermesRuntime;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::impls::birelay::components::CosmosBiRelayComponents;

impl HasTwoChainTypes for CosmosBiRelay {
    type ChainA = CosmosChain;

    type ChainB = CosmosChain;
}

impl HasTwoWayRelayTypes for CosmosBiRelay {
    type RelayAToB = CosmosRelay;

    type RelayBToA = CosmosRelay;
}

impl HasTwoWayRelay for CosmosBiRelay {
    fn relay_a_to_b(&self) -> &CosmosRelay {
        &self.relay_a_to_b
    }

    fn relay_b_to_a(&self) -> &CosmosRelay {
        &self.relay_b_to_a
    }
}

impl ProvideRuntime<CosmosBiRelay> for CosmosBiRelayComponents {
    fn runtime(birelay: &CosmosBiRelay) -> &HermesRuntime {
        &birelay.runtime
    }
}
