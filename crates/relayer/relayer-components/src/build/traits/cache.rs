use alloc::collections::BTreeMap;

use hermes_runtime_components::traits::mutex::{HasMutex, MutexOf};
use hermes_runtime_components::traits::runtime::{HasRuntime, RuntimeOf};

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::multi::traits::chain_at::{ChainAt, ChainIdAt, HasChainTypeAt};
use crate::multi::traits::relay_at::{ClientIdAt, HasBoundedRelayTypeAt, RelayAt};

pub trait HasChainCache<I>:
    HasChainTypeAt<I, Chain: HasChainIdType> + HasRuntime<Runtime: HasMutex>
{
    fn chain_cache(&self) -> &ChainCacheAt<Self, I>;
}

pub trait HasRelayCache<Src, Dst>:
    HasBoundedRelayTypeAt<Src, Dst> + HasRuntime<Runtime: HasMutex>
{
    fn relay_cache(&self) -> &RelayCacheAt<Self, Src, Dst>;
}

pub type RelayCacheAt<Build, Src, Dst> = MutexOf<
    RuntimeOf<Build>,
    BTreeMap<
        (
            ChainIdAt<Build, Src>,
            ChainIdAt<Build, Dst>,
            ClientIdAt<Build, Src, Dst>,
            ClientIdAt<Build, Dst, Src>,
        ),
        RelayAt<Build, Src, Dst>,
    >,
>;

pub type ChainCacheAt<Build, I> =
    MutexOf<RuntimeOf<Build>, BTreeMap<ChainIdAt<Build, I>, ChainAt<Build, I>>>;
