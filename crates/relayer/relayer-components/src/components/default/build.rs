use cgp::core::error::ProvideErrorType;
use cgp::prelude::*;

use crate::build::components::birelay::BuildBiRelayFromRelays;
use crate::build::components::chain::cache::BuildChainWithCache;
use crate::build::components::relay::build_from_chain::BuildRelayFromChains;
use crate::build::components::relay::cache::BuildRelayWithCache;
use crate::build::traits::builders::birelay_builder::{BiRelayBuilderComponent, CanBuildBiRelay};
use crate::build::traits::builders::birelay_from_relay_builder::BiRelayFromRelayBuilder;
use crate::build::traits::builders::chain_builder::{ChainBuilder, ChainBuilderComponent};
use crate::build::traits::builders::relay_builder::RelayBuilderComponent;
use crate::build::traits::builders::relay_from_chains_builder::RelayFromChainsBuilder;
use crate::build::traits::cache::{HasChainCache, HasRelayCache};
use crate::multi::traits::birelay_at::HasBiRelayTypeAt;
use crate::multi::traits::chain_at::{ChainAt, ChainIdAt};
use crate::multi::traits::relay_at::{ClientIdAt, RelayAt};
use crate::multi::types::index::Index;

cgp_preset! {
    DefaultBuildComponents<BaseComponents: Async> {
        ChainBuilderComponent: BuildChainWithCache<BaseComponents>,
        RelayBuilderComponent: BuildRelayWithCache<BuildRelayFromChains>,
        BiRelayBuilderComponent: BuildBiRelayFromRelays,
    }
}

pub trait CanUseDefaultBuildComponents: UseDefaultBuildComponents {}

pub trait UseDefaultBuildComponents: CanBuildBiRelay<Index<0>, Index<1>> {}

impl<Build, Components, BaseComponents> UseDefaultBuildComponents for Build
where
    Build: Async
        + HasBiRelayTypeAt<Index<0>, Index<1>>
        + HasRelayCache<Index<0>, Index<1>>
        + HasRelayCache<Index<1>, Index<0>>
        + HasChainCache<Index<0>>
        + HasChainCache<Index<1>>
        + HasComponents<Components = Components>,
    RelayAt<Build, Index<0>, Index<1>>: Clone,
    RelayAt<Build, Index<1>, Index<0>>: Clone,
    ChainAt<Build, Index<0>>: Clone,
    ChainAt<Build, Index<1>>: Clone,
    ChainIdAt<Build, Index<0>>: Ord + Clone,
    ChainIdAt<Build, Index<1>>: Ord + Clone,
    ClientIdAt<Build, Index<0>, Index<1>>: Ord + Clone,
    ClientIdAt<Build, Index<1>, Index<0>>: Ord + Clone,
    Components: HasComponents<Components = BaseComponents>
        + DelegatesToDefaultBuildComponents<BaseComponents>
        + BiRelayFromRelayBuilder<Build, Index<0>, Index<1>>
        + RelayFromChainsBuilder<Build, Index<0>, Index<1>>
        + RelayFromChainsBuilder<Build, Index<1>, Index<0>>
        + ProvideErrorType<Build, Error: Async>,
    BaseComponents: Async + ChainBuilder<Build, Index<0>> + ChainBuilder<Build, Index<1>>,
{
}
