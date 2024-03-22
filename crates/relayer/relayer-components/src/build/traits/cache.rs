use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::target::chain::ChainBuildTarget;
use crate::build::traits::target::relay::RelayBuildTarget;
use crate::build::types::aliases::{TargetChainCache, TargetRelayCache};
use hermes_runtime_components::traits::mutex::HasMutex;
use hermes_runtime_components::traits::runtime::HasRuntime;

pub trait HasChainCache<Target>: HasBiRelayType + HasRuntime
where
    Target: ChainBuildTarget<Self>,
    Self::Runtime: HasMutex,
{
    fn chain_cache(&self) -> &TargetChainCache<Self, Target>;
}

pub trait HasRelayCache<Target>: HasBiRelayType + HasRuntime
where
    Target: RelayBuildTarget<Self>,
    Self::Runtime: HasMutex,
{
    fn relay_cache(&self) -> &TargetRelayCache<Self, Target>;
}
