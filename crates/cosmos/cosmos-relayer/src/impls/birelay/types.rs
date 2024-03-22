use hermes_relayer_components::birelay::traits::two_way::{
    ProvideTwoChainTypes, ProvideTwoWayRelayTypes, TwoWayRelayGetter,
};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeGetter;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::impls::birelay::components::CosmosBiRelayComponents;

impl ProvideTwoChainTypes<CosmosBiRelay> for CosmosBiRelayComponents {
    type ChainA = CosmosChain;

    type ChainB = CosmosChain;
}

impl ProvideTwoWayRelayTypes<CosmosBiRelay> for CosmosBiRelayComponents {
    type RelayAToB = CosmosRelay;

    type RelayBToA = CosmosRelay;
}

impl TwoWayRelayGetter<CosmosBiRelay> for CosmosBiRelayComponents {
    fn relay_a_to_b(birelay: &CosmosBiRelay) -> &CosmosRelay {
        &birelay.relay_a_to_b
    }

    fn relay_b_to_a(birelay: &CosmosBiRelay) -> &CosmosRelay {
        &birelay.relay_b_to_a
    }
}

impl RuntimeGetter<CosmosBiRelay> for CosmosBiRelayComponents {
    fn runtime(birelay: &CosmosBiRelay) -> &HermesRuntime {
        &birelay.runtime
    }
}
