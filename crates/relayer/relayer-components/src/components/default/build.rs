use cgp::core::error::ProvideErrorType;
use cgp::core::field::Index;
use cgp::prelude::*;

use crate::build::components::birelay::BuildBiRelayFromRelays;
use crate::build::components::chain::cache::BuildChainWithCache;
use crate::build::components::relay::build_from_chain::BuildRelayFromChains;
use crate::build::components::relay::cache::BuildRelayWithCache;
use crate::build::traits::builders::birelay_builder::{BiRelayBuilderComponent, CanBuildBiRelay};
use crate::build::traits::builders::birelay_from_relay_builder::{BiRelayFromRelayBuilder, CanBuildBiRelayFromRelays};
use crate::build::traits::builders::chain_builder::{CanBuildChain, ChainBuilder, ChainBuilderComponent};
use crate::build::traits::builders::relay_builder::RelayBuilderComponent;
use crate::build::traits::builders::relay_from_chains_builder::{CanBuildRelayFromChains, RelayFromChainsBuilder};
use crate::build::traits::cache::{HasChainCache, HasRelayCache};
use crate::multi::traits::birelay_at::HasBiRelayTypeAt;
use crate::multi::traits::chain_at::{ChainAt, ChainIdAt};
use crate::multi::traits::relay_at::{ClientIdAt, RelayAt};

cgp_preset! {
    DefaultBuildComponents<BaseComponents: Async> {
        ChainBuilderComponent: BuildChainWithCache<BaseComponents>,
        RelayBuilderComponent: BuildRelayWithCache<BuildRelayFromChains>,
        BiRelayBuilderComponent: BuildBiRelayFromRelays,
    }
}

pub trait CanUseDefaultBuildComponents: UseDefaultBuildComponents {}

pub trait UseDefaultBuildComponents: CanBuildBiRelay<Index<0>, Index<1>> {}

impl<Build> UseDefaultBuildComponents for Build
where
    Build: Async
        + HasBiRelayTypeAt<Index<0>, Index<1>>
        + HasRelayCache<Index<0>, Index<1>>
        + HasRelayCache<Index<1>, Index<0>>
        + HasChainCache<Index<0>>
        + HasChainCache<Index<1>>
        + CanBuildChain<Index<0>> + ChainBuilder<Build, Index<1>>
        + CanBuildBiRelayFromRelays<Index<0>, Index<1>>
        + CanBuildRelayFromChains<Index<0>, Index<1>>
        + CanBuildRelayFromChains<Index<1>, Index<0>>,
    RelayAt<Build, Index<0>, Index<1>>: Clone,
    RelayAt<Build, Index<1>, Index<0>>: Clone,
    ChainAt<Build, Index<0>>: Clone,
    ChainAt<Build, Index<1>>: Clone,
    ChainIdAt<Build, Index<0>>: Ord + Clone,
    ChainIdAt<Build, Index<1>>: Ord + Clone,
    ClientIdAt<Build, Index<0>, Index<1>>: Ord + Clone,
    ClientIdAt<Build, Index<1>, Index<0>>: Ord + Clone,
{
}
