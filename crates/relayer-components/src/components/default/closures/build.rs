use cgp_core::{HasComponents, HasErrorType};

use crate::birelay::traits::two_way::{HasTwoChainTypes, HasTwoWayRelay};
use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::cache::{HasChainCache, HasRelayCache};
use crate::build::traits::components::birelay_builder::CanBuildBiRelay;
use crate::build::traits::components::birelay_from_relay_builder::BiRelayFromRelayBuilder;
use crate::build::traits::components::chain_builder::ChainBuilder;
use crate::build::traits::components::relay_from_chains_builder::RelayFromChainsBuilder;
use crate::build::traits::target::chain::{ChainATarget, ChainBTarget};
use crate::build::traits::target::relay::{RelayAToBTarget, RelayBToATarget};
use crate::build::types::aliases::{ChainA, ChainB};
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::components::default::build::DelegatesToDefaultBuildComponents;
use crate::relay::traits::chains::HasRelayChains;
use crate::runtime::traits::mutex::HasMutex;

pub trait UseDefaultBuildComponents<BaseComponents>: CanBuildBiRelay
where
    ChainA<Self>: HasIbcChainTypes<ChainB<Self>>,
    ChainB<Self>: HasIbcChainTypes<ChainA<Self>>,
{
}

impl<Build, BiRelay, RelayAToB, RelayBToA, ChainA, ChainB, Components, BaseComponents>
    UseDefaultBuildComponents<BaseComponents> for Build
where
    Build: HasErrorType
        + HasBiRelayType<BiRelay = BiRelay>
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
    Build::Runtime: HasMutex,
    Components: DelegatesToDefaultBuildComponents<BaseComponents>
        + BiRelayFromRelayBuilder<Build>
        + RelayFromChainsBuilder<Build, RelayAToBTarget>
        + RelayFromChainsBuilder<Build, RelayBToATarget>,
    BaseComponents: ChainBuilder<Build, ChainATarget> + ChainBuilder<Build, ChainBTarget>,
{
}
