use hermes_runtime_components::traits::mutex::HasMutex;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::target::chain::ChainBuildTarget;
use crate::build::traits::target::relay::RelayBuildTarget;
use crate::build::types::aliases::{TargetChainCache, TargetRelayCache};

pub trait HasChainCache<Target>: HasBiRelayType + HasRuntime<Runtime: HasMutex>
where
    Target: ChainBuildTarget<Self>,
{
    fn chain_cache(&self) -> &TargetChainCache<Self, Target>;
}

pub trait HasRelayCache<Target>: HasBiRelayType + HasRuntime<Runtime: HasMutex>
where
    Target: RelayBuildTarget<Self>,
{
    fn relay_cache(&self) -> &TargetRelayCache<Self, Target>;
}
