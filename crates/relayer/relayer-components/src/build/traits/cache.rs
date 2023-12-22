use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::target::chain::ChainBuildTarget;
use crate::build::traits::target::relay::RelayBuildTarget;
use crate::build::types::aliases::{TargetChainCache, TargetRelayCache};
use crate::runtime::traits::mutex::HasMutex;
use crate::runtime::traits::runtime::HasRuntime;

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
