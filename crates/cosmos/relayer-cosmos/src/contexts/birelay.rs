use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer::chain::handle::ChainHandle;

use crate::contexts::relay::CosmosRelay;

#[derive(Clone)]
pub struct CosmosBiRelay<ChainA, ChainB> {
    pub runtime: HermesRuntime,
    pub relay_a_to_b: CosmosRelay<ChainA, ChainB>,
    pub relay_b_to_a: CosmosRelay<ChainB, ChainA>,
}

impl<ChainA, ChainB> CosmosBiRelay<ChainA, ChainB>
where
    ChainA: ChainHandle,
    ChainB: ChainHandle,
{
    pub fn new(
        runtime: HermesRuntime,
        relay_a_to_b: CosmosRelay<ChainA, ChainB>,
        relay_b_to_a: CosmosRelay<ChainB, ChainA>,
    ) -> Self {
        Self {
            runtime,
            relay_a_to_b,
            relay_b_to_a,
        }
    }
}
