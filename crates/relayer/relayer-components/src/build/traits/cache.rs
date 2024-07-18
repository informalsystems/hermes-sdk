use hermes_runtime_components::traits::mutex::HasMutex;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::build::types::aliases::{ChainCacheAt, RelayCacheAt};
use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::multi::traits::chain_at::HasChainTypeAt;
use crate::multi::traits::relay_at::HasRelayTypeAt;

pub trait HasChainCache<const I: usize>:
    HasChainTypeAt<I, Chain: HasChainIdType> + HasRuntime<Runtime: HasMutex>
{
    fn chain_cache(&self) -> &ChainCacheAt<Self, I>;
}

pub trait HasRelayCache<const SRC: usize, const DST: usize>:
    HasRelayTypeAt<SRC, DST> + HasRuntime<Runtime: HasMutex>
{
    fn relay_cache(&self) -> &RelayCacheAt<Self, SRC, DST>;
}
