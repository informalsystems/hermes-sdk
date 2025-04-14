use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use core::marker::PhantomData;

use cgp::core::field::UseField;
use cgp::core::Async;
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_chain_type_components::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::chain::types::aliases::{ChainIdOf, ClientIdOf};
use hermes_relayer_components::multi::traits::relay_at::HasRelayTypeAt;
use hermes_relayer_components::relay::traits::target::{
    CounterpartyChainOf, HasTargetChainTypes, RelayTarget, TargetChainOf,
};

use crate::batch::traits::types::{HasMessageBatchChannelTypes, MessageBatchSenderOf};

pub trait HasBatchSenderCacheType<SrcTag: Async, DstTag: Async, Target: RelayTarget>:
    Async
{
    type BatchSenderCache;
}

impl<Build, SrcTag: Async, DstTag: Async, Target: RelayTarget, Relay>
    HasBatchSenderCacheType<SrcTag, DstTag, Target> for Build
where
    Build: HasRelayTypeAt<SrcTag, DstTag, Relay = Relay>,
    Relay: HasMessageBatchChannelTypes<Target::Chain>
        + HasTargetChainTypes<
            Target,
            TargetChain: HasChainIdType + HasClientIdType<Relay::CounterpartyChain>,
            CounterpartyChain: HasChainIdType + HasClientIdType<Relay::TargetChain>,
        >,
{
    type BatchSenderCache = Arc<
        Mutex<
            BTreeMap<
                (
                    ChainIdOf<Relay::TargetChain>,
                    ChainIdOf<Relay::CounterpartyChain>,
                    ClientIdOf<Relay::TargetChain, Relay::CounterpartyChain>,
                    ClientIdOf<Relay::CounterpartyChain, Relay::TargetChain>,
                ),
                MessageBatchSenderOf<Relay, Target::Chain>,
            >,
        >,
    >;
}

pub type BatchSenderCacheAt<Build, SrcTag, DstTag, Target> =
    <Build as HasBatchSenderCacheType<SrcTag, DstTag, Target>>::BatchSenderCache;

pub trait CanUseBatchSenderCache<SrcTag: Async, DstTag: Async, Target: RelayTarget>:
    HasRelayTypeAt<
        SrcTag,
        DstTag,
        Relay: HasMessageBatchChannelTypes<Target::Chain>
                   + HasTargetChainTypes<
            Target,
            TargetChain: HasChainIdType + HasClientIdType<CounterpartyChainOf<Self::Relay, Target>>,
            CounterpartyChain: HasChainIdType + HasClientIdType<TargetChainOf<Self::Relay, Target>>,
        >,
    > + HasBatchSenderCacheType<
        SrcTag,
        DstTag,
        Target,
        BatchSenderCache = Arc<
            Mutex<
                BTreeMap<
                    (
                        ChainIdOf<TargetChainOf<Self::Relay, Target>>,
                        ChainIdOf<CounterpartyChainOf<Self::Relay, Target>>,
                        ClientIdOf<
                            TargetChainOf<Self::Relay, Target>,
                            CounterpartyChainOf<Self::Relay, Target>,
                        >,
                        ClientIdOf<
                            CounterpartyChainOf<Self::Relay, Target>,
                            TargetChainOf<Self::Relay, Target>,
                        >,
                    ),
                    MessageBatchSenderOf<Self::Relay, Target::Chain>,
                >,
            >,
        >,
    >
{
}

impl<Build, SrcTag: Async, DstTag: Async, Target: RelayTarget, Relay>
    CanUseBatchSenderCache<SrcTag, DstTag, Target> for Build
where
    Build: HasRelayTypeAt<SrcTag, DstTag, Relay = Relay>,
    Relay: HasMessageBatchChannelTypes<Target::Chain>
        + HasTargetChainTypes<
            Target,
            TargetChain: HasChainIdType + HasClientIdType<Relay::CounterpartyChain>,
            CounterpartyChain: HasChainIdType + HasClientIdType<Relay::TargetChain>,
        >,
{
}

#[cgp_component {
  provider: BatchSenderCacheGetter,
  context: Build,
}]
pub trait HasBatchSenderCache<SrcTag: Async, DstTag: Async, Target: RelayTarget>:
    HasBatchSenderCacheType<SrcTag, DstTag, Target>
{
    fn batch_sender_cache(
        &self,
        _tag: PhantomData<(SrcTag, DstTag, Target)>,
    ) -> &Self::BatchSenderCache;
}

#[cgp_provider(BatchSenderCacheGetterComponent)]
impl<Build, SrcTag: Async, DstTag: Async, Target: RelayTarget, FieldTag>
    BatchSenderCacheGetter<Build, SrcTag, DstTag, Target> for UseField<FieldTag>
where
    Build: HasBatchSenderCacheType<SrcTag, DstTag, Target>
        + HasField<FieldTag, Value = Build::BatchSenderCache>,
{
    fn batch_sender_cache(
        build: &Build,
        _tag: PhantomData<(SrcTag, DstTag, Target)>,
    ) -> &Build::BatchSenderCache {
        build.get_field(PhantomData)
    }
}
