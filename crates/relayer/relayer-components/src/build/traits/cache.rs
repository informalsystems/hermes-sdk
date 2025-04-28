use alloc::collections::BTreeMap;

use futures::lock::Mutex;

use crate::chain::traits::HasChainIdType;
use crate::multi::traits::chain_at::{ChainAt, ChainIdAt, HasChainTypeAt};
use crate::multi::traits::relay_at::{ClientIdAt, HasBoundedRelayTypeAt, RelayAt};

pub trait HasChainCache<I>: HasChainTypeAt<I, Chain: HasChainIdType> {
    fn chain_cache(&self) -> &ChainCacheAt<Self, I>;
}

pub trait HasRelayCache<Src, Dst>: HasBoundedRelayTypeAt<Src, Dst> {
    fn relay_cache(&self) -> &RelayCacheAt<Self, Src, Dst>;
}

pub type RelayCacheAt<Build, Src, Dst> = Mutex<
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

pub type ChainCacheAt<Build, I> = Mutex<BTreeMap<ChainIdAt<Build, I>, ChainAt<Build, I>>>;
