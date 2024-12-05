use alloc::collections::BTreeMap;
use core::marker::PhantomData;

use cgp::core::Async;
use hermes_chain_type_components::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, ChainIdAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::{ClientIdAt, HasRelayTypeAt};
use hermes_relayer_components::relay::traits::target::RelayTarget;
use hermes_runtime_components::traits::mutex::{HasMutex, MutexOf};
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::batch::traits::types::{HasMessageBatchChannelTypes, MessageBatchSenderOf};

pub trait HasBatchSenderCacheType<SrcTag: Async, DstTag: Async, Target: RelayTarget>:
    Async
{
    type BatchSenderCache;
}

impl<Build, SrcTag: Async, DstTag: Async, Target: RelayTarget>
    HasBatchSenderCacheType<SrcTag, DstTag, Target> for Build
where
    Build: HasRuntime<Runtime: HasMutex>
        + HasChainTypeAt<SrcTag, Chain: HasChainIdType + HasClientIdType<ChainAt<Build, DstTag>>>
        + HasChainTypeAt<DstTag, Chain: HasChainIdType + HasClientIdType<ChainAt<Build, SrcTag>>>
        + HasRelayTypeAt<SrcTag, DstTag, Relay: HasMessageBatchChannelTypes<Target::Chain>>,
{
    type BatchSenderCache = MutexOf<
        Build::Runtime,
        BTreeMap<
            (
                ChainIdAt<Build, SrcTag>,
                ChainIdAt<Build, DstTag>,
                ClientIdAt<Build, SrcTag, DstTag>,
                ClientIdAt<Build, DstTag, SrcTag>,
            ),
            MessageBatchSenderOf<Build::Relay, Target::Chain>,
        >,
    >;
}

pub trait CanUseBatchSenderCacheType<SrcTag: Async, DstTag: Async, Target: RelayTarget>:
    HasRuntime<Runtime: HasMutex>
    + HasChainTypeAt<SrcTag, Chain: HasChainIdType + HasClientIdType<ChainAt<Self, DstTag>>>
    + HasChainTypeAt<DstTag, Chain: HasChainIdType + HasClientIdType<ChainAt<Self, SrcTag>>>
    + HasRelayTypeAt<SrcTag, DstTag, Relay: HasMessageBatchChannelTypes<Target::Chain>>
    + HasBatchSenderCacheType<
        SrcTag,
        DstTag,
        Target,
        BatchSenderCache = MutexOf<
            Self::Runtime,
            BTreeMap<
                (
                    ChainIdAt<Self, SrcTag>,
                    ChainIdAt<Self, DstTag>,
                    ClientIdAt<Self, SrcTag, DstTag>,
                    ClientIdAt<Self, DstTag, SrcTag>,
                ),
                MessageBatchSenderOf<Self::Relay, Target::Chain>,
            >,
        >,
    >
{
}

impl<Build, SrcTag: Async, DstTag: Async, Target: RelayTarget>
    CanUseBatchSenderCacheType<SrcTag, DstTag, Target> for Build
where
    Build: HasRuntime<Runtime: HasMutex>
        + HasChainTypeAt<SrcTag, Chain: HasChainIdType + HasClientIdType<ChainAt<Build, DstTag>>>
        + HasChainTypeAt<DstTag, Chain: HasChainIdType + HasClientIdType<ChainAt<Build, SrcTag>>>
        + HasRelayTypeAt<SrcTag, DstTag, Relay: HasMessageBatchChannelTypes<Target::Chain>>,
{
}

pub trait HasBatchSenderCache<SrcTag: Async, DstTag: Async, Target: RelayTarget>:
    HasBatchSenderCacheType<SrcTag, DstTag, Target>
{
    fn batch_sender_cache(
        &self,
        _tag: PhantomData<(SrcTag, DstTag, Target)>,
    ) -> &Self::BatchSenderCache;
}
