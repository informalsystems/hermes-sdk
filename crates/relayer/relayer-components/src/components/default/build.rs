use cgp_core::error::ProvideErrorType;
use cgp_core::prelude::*;

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
use crate::multi::traits::chain_at::{ChainIdAt, ChainAt};
use crate::multi::traits::relay_at::{ClientIdAt, RelayTypeAt};

define_components! {
    DefaultBuildComponents<BaseComponents: Async> {
        ChainBuilderComponent: BuildChainWithCache<BaseComponents>,
        RelayBuilderComponent: BuildRelayWithCache<BuildRelayFromChains>,
        BiRelayBuilderComponent: BuildBiRelayFromRelays,
    }
}

pub trait CanUseDefaultBuildComponents: UseDefaultBuildComponents {}

pub trait UseDefaultBuildComponents: CanBuildBiRelay<0, 1> {}

impl<Build, Components, BaseComponents> UseDefaultBuildComponents for Build
where
    Build: HasBiRelayTypeAt<0, 1>
        + HasRelayCache<0, 1>
        + HasRelayCache<1, 0>
        + HasChainCache<0>
        + HasChainCache<1>
        + HasComponents<Components = Components>,
    RelayTypeAt<Build, 0, 1>: Clone,
    RelayTypeAt<Build, 1, 0>: Clone,
    ChainAt<Build, 0>: Clone,
    ChainAt<Build, 1>: Clone,
    ChainIdAt<Build, 0>: Ord + Clone,
    ChainIdAt<Build, 1>: Ord + Clone,
    ClientIdAt<Build, 0, 1>: Ord + Clone,
    ClientIdAt<Build, 1, 0>: Ord + Clone,
    Components: HasComponents<Components = BaseComponents>
        + DelegatesToDefaultBuildComponents<BaseComponents>
        + BiRelayFromRelayBuilder<Build, 0, 1>
        + RelayFromChainsBuilder<Build, 0, 1>
        + RelayFromChainsBuilder<Build, 1, 0>
        + ProvideErrorType<Build>,
    BaseComponents: ChainBuilder<Build, 0> + ChainBuilder<Build, 1>,
{
}
