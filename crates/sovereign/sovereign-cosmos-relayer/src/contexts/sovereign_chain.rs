use cgp_core::prelude::*;
use hermes_celestia_chain::contexts::celestia_chain::CelestiaChain;

pub struct SovereignChain {
    pub celestia_chain: CelestiaChain,
    // TODO: fields such as rollup JSON RPC address
}

pub struct SovereignChainComponents;

impl HasComponents for SovereignChain {
    type Components = SovereignChainComponents;
}
