use cgp_core::Async;

use crate::build::traits::birelay::HasBiRelayType;
use crate::build::types::aliases::{ChainA, ChainB};
use crate::chain::traits::types::ibc::HasIbcChainTypes;

#[derive(Default)]
pub struct ChainATarget;

#[derive(Default)]
pub struct ChainBTarget;

pub trait ChainBuildTarget<Build>: Default + Async {
    type TargetChain: HasIbcChainTypes<Self::CounterpartyChain>;

    type CounterpartyChain: HasIbcChainTypes<Self::TargetChain>;
}

impl<Build> ChainBuildTarget<Build> for ChainATarget
where
    Build: HasBiRelayType,
    ChainA<Build>: HasIbcChainTypes<ChainB<Build>>,
    ChainB<Build>: HasIbcChainTypes<ChainA<Build>>,
{
    type TargetChain = ChainA<Build>;

    type CounterpartyChain = ChainB<Build>;
}

impl<Build> ChainBuildTarget<Build> for ChainBTarget
where
    Build: HasBiRelayType,
    ChainA<Build>: HasIbcChainTypes<ChainB<Build>>,
    ChainB<Build>: HasIbcChainTypes<ChainA<Build>>,
{
    type TargetChain = ChainB<Build>;

    type CounterpartyChain = ChainA<Build>;
}
