use cgp_core::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;

use crate::contexts::sovereign_chain::SovereignChain;

pub struct CosmosToSovereignRelay {
    pub sovereign_chain: SovereignChain,
    pub cosmos_chain: CosmosChain,
    // TODO: Relay fields
}

pub struct CosmosToSovereignRelayComponents;

impl HasComponents for CosmosToSovereignRelay {
    type Components = CosmosToSovereignRelayComponents;
}
