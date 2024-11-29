use alloc::collections::BTreeMap;

use hermes_runtime_components::traits::mutex::{HasMutex, MutexOf};
use hermes_runtime_components::traits::runtime::{HasRuntime, RuntimeOf};

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::multi::traits::chain_at::{ChainAt, ChainIdAt, HasChainTypeAt};
use crate::multi::traits::relay_at::{ClientIdAt, HasBoundedRelayTypeAt, RelayAt};

pub trait HasChainCache<const I: usize>:
    HasChainTypeAt<I, Chain: HasChainIdType> + HasRuntime<Runtime: HasMutex>
{
    fn chain_cache(&self) -> &ChainCacheAt<Self, I>;
}

pub trait HasRelayCache<const SRC: usize, const DST: usize>:
    HasBoundedRelayTypeAt<SRC, DST> + HasRuntime<Runtime: HasMutex>
{
    fn relay_cache(&self) -> &RelayCacheAt<Self, SRC, DST>;
}

pub type RelayCacheAt<Build, const SRC: usize, const DST: usize> = MutexOf<
    RuntimeOf<Build>,
    BTreeMap<
        (
            ChainIdAt<Build, SRC>,
            ChainIdAt<Build, DST>,
            ClientIdAt<Build, SRC, DST>,
            ClientIdAt<Build, DST, SRC>,
        ),
        RelayAt<Build, SRC, DST>,
    >,
>;

pub type ChainCacheAt<Build, const I: usize> =
    MutexOf<RuntimeOf<Build>, BTreeMap<ChainIdAt<Build, I>, ChainAt<Build, I>>>;
