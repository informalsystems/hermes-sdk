use cgp_core::prelude::*;

use crate::contexts::cosmos_to_sovereign_relay::CosmosToSovereignRelay;
use crate::contexts::sovereign_to_cosmos_relay::SovereignToCosmosRelay;

pub struct SovereignCosmosBiRelay {
    pub sovereign_to_cosmos_relay: SovereignToCosmosRelay,
    pub cosmos_to_sovereign_relay: CosmosToSovereignRelay,
}

pub struct SovereignCosmosBiRelayComponents;

impl HasComponents for SovereignCosmosBiRelay {
    type Components = SovereignCosmosBiRelayComponents;
}
