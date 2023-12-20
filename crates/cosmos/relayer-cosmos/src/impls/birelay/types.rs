use cgp_core::Async;
use hermes_relayer_components::birelay::traits::two_way::{
    HasTwoChainTypes, HasTwoWayRelay, HasTwoWayRelayTypes,
};
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::impls::birelay::components::CosmosBiRelayComponents;

impl<ChainA, ChainB> HasTwoChainTypes for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: ChainHandle,
    ChainB: ChainHandle,
{
    type ChainA = CosmosChain<ChainA>;

    type ChainB = CosmosChain<ChainB>;
}

impl<ChainA, ChainB> HasTwoWayRelayTypes for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: ChainHandle,
    ChainB: ChainHandle,
{
    type RelayAToB = CosmosRelay<ChainA, ChainB>;

    type RelayBToA = CosmosRelay<ChainB, ChainA>;
}

impl<ChainA, ChainB> HasTwoWayRelay for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: ChainHandle,
    ChainB: ChainHandle,
{
    fn relay_a_to_b(&self) -> &CosmosRelay<ChainA, ChainB> {
        &self.relay_a_to_b
    }

    fn relay_b_to_a(&self) -> &CosmosRelay<ChainB, ChainA> {
        &self.relay_b_to_a
    }
}

impl<ChainA, ChainB> ProvideRuntime<CosmosBiRelay<ChainA, ChainB>> for CosmosBiRelayComponents
where
    ChainA: Async,
    ChainB: Async,
{
    fn runtime(birelay: &CosmosBiRelay<ChainA, ChainB>) -> &TokioRuntimeContext {
        &birelay.runtime
    }
}
