use cgp_core::prelude::*;

use crate::birelay::traits::two_way::{HasTwoChainTypes, HasTwoWayRelay};
use crate::build::components::birelay::BuildBiRelayFromRelays;
use crate::build::components::chain::cache::BuildChainWithCache;
use crate::build::components::relay::build_from_chain::BuildRelayFromChains;
use crate::build::components::relay::cache::BuildRelayWithCache;
use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::builders::birelay_builder::{BiRelayBuilderComponent, CanBuildBiRelay};
use crate::build::traits::builders::birelay_from_relay_builder::BiRelayFromRelayBuilder;
use crate::build::traits::builders::chain_builder::{ChainBuilder, ChainBuilderComponent};
use crate::build::traits::builders::relay_builder::RelayBuilderComponent;
use crate::build::traits::builders::relay_from_chains_builder::RelayFromChainsBuilder;
use crate::build::traits::cache::{HasChainCache, HasRelayCache};
use crate::build::traits::target::chain::{ChainATarget, ChainBTarget};
use crate::build::traits::target::relay::{RelayAToBTarget, RelayBToATarget};
use crate::build::types::aliases::{ChainA, ChainB};
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::relay::traits::chains::HasRelayChains;

define_components! {
    DefaultBuildComponents<BaseComponents: Async> {
        ChainBuilderComponent: BuildChainWithCache<BaseComponents>,
        RelayBuilderComponent: BuildRelayWithCache<BuildRelayFromChains>,
        BiRelayBuilderComponent: BuildBiRelayFromRelays,
    }
}

pub trait CanUseDefaultBuildComponents: UseDefaultBuildComponents
where
    ChainA<Self>: HasIbcChainTypes<ChainB<Self>>,
    ChainB<Self>: HasIbcChainTypes<ChainA<Self>>,
{
}

pub trait UseDefaultBuildComponents: CanBuildBiRelay
where
    ChainA<Self>: HasIbcChainTypes<ChainB<Self>>,
    ChainB<Self>: HasIbcChainTypes<ChainA<Self>>,
{
}

impl<Build, BiRelay, RelayAToB, RelayBToA, ChainA, ChainB, Components, BaseComponents>
    UseDefaultBuildComponents for Build
where
    Build: HasBiRelayType<BiRelay = BiRelay>
        + HasRelayCache<RelayAToBTarget>
        + HasRelayCache<RelayBToATarget>
        + HasChainCache<ChainATarget>
        + HasChainCache<ChainBTarget>
        + HasComponents<Components = Components>,
    BiRelay: HasTwoChainTypes<ChainA = ChainA, ChainB = ChainB>
        + HasTwoWayRelay<RelayAToB = RelayAToB, RelayBToA = RelayBToA>,
    RelayAToB: Clone + HasRelayChains<SrcChain = ChainA, DstChain = ChainB>,
    RelayBToA: Clone + HasRelayChains<SrcChain = ChainB, DstChain = ChainA>,
    ChainA: Clone + HasIbcChainTypes<ChainB> + HasErrorType,
    ChainB: Clone + HasIbcChainTypes<ChainA> + HasErrorType,
    ChainA::ChainId: Ord + Clone,
    ChainB::ChainId: Ord + Clone,
    ChainA::ClientId: Ord + Clone,
    ChainB::ClientId: Ord + Clone,
    Components: HasComponents<Components = BaseComponents>
        + DelegatesToDefaultBuildComponents<BaseComponents>
        + BiRelayFromRelayBuilder<Build>
        + RelayFromChainsBuilder<Build, RelayAToBTarget>
        + RelayFromChainsBuilder<Build, RelayBToATarget>,
    BaseComponents: ChainBuilder<Build, ChainATarget> + ChainBuilder<Build, ChainBTarget>,
{
}
