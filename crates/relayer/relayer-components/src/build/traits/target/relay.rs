use cgp_core::Async;

use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::target::chain::{ChainATarget, ChainBTarget, ChainBuildTarget};
use crate::build::types::aliases::{ChainA, ChainB, RelayAToB, RelayBToA, RelayError};
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::relay::traits::chains::HasRelayChains;

#[derive(Default)]
pub struct RelayAToBTarget;

#[derive(Default)]
pub struct RelayBToATarget;

pub trait RelayBuildTarget<Build>: Default + Async
where
    Build: HasBiRelayType,
{
    type TargetRelay: HasRelayChains<Error = RelayError<Build>>;

    type SrcChainTarget: ChainBuildTarget<
        Build,
        TargetChain = <Self::TargetRelay as HasRelayChains>::SrcChain,
        CounterpartyChain = <Self::TargetRelay as HasRelayChains>::DstChain,
    >;

    type DstChainTarget: ChainBuildTarget<
        Build,
        TargetChain = <Self::TargetRelay as HasRelayChains>::DstChain,
        CounterpartyChain = <Self::TargetRelay as HasRelayChains>::SrcChain,
    >;
}

impl<Build> RelayBuildTarget<Build> for RelayAToBTarget
where
    Build: HasBiRelayType,
    ChainA<Build>: HasIbcChainTypes<ChainB<Build>>,
    ChainB<Build>: HasIbcChainTypes<ChainA<Build>>,
{
    type TargetRelay = RelayAToB<Build>;

    type SrcChainTarget = ChainATarget;

    type DstChainTarget = ChainBTarget;
}

impl<Build> RelayBuildTarget<Build> for RelayBToATarget
where
    Build: HasBiRelayType,
    ChainA<Build>: HasIbcChainTypes<ChainB<Build>>,
    ChainB<Build>: HasIbcChainTypes<ChainA<Build>>,
{
    type TargetRelay = RelayBToA<Build>;

    type SrcChainTarget = ChainBTarget;

    type DstChainTarget = ChainATarget;
}
