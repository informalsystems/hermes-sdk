use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;

pub struct CelestiaChain {
    pub cosmos_chain: CosmosChain,
    // TOOD: extra fields for querying Celestia full node
}

pub struct CelestiaChainComponents;

impl HasComponents for CelestiaChain {
    type Components = CelestiaChainComponents;
}
