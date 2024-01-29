use alloc::collections::BTreeMap;

use cgp_core::Async;
use hermes_relayer_components::build::traits::birelay::HasBiRelayType;
use hermes_relayer_components::build::traits::target::chain::ChainBuildTarget;
use hermes_relayer_components::build::types::aliases::{
    CounterpartyChainId, CounterpartyClientId, TargetChain, TargetChainId, TargetClientId,
};
use hermes_relayer_components::runtime::traits::mutex::{HasMutex, MutexOf};
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_relayer_components::runtime::types::aliases::RuntimeOf;

use crate::batch::traits::channel::HasMessageBatchSenderType;

pub trait HasBatchSenderCache<Target, Error>: Async
where
    Target: HasBatchSenderCacheType<Self, Error>,
{
    fn batch_sender_cache(&self, target: Target) -> &Target::BatchSenderCache;
}

pub trait HasBatchSenderCacheType<Build, Error>: Async {
    type BatchSenderCache: Async;
}

impl<Target, Build, Error> HasBatchSenderCacheType<Build, Error> for Target
where
    Error: Async,
    Build: HasBiRelayType + HasRuntime,
    Target: ChainBuildTarget<Build>,
    Target::TargetChain: HasMessageBatchSenderType<Error>,
    Build::Runtime: HasMutex,
{
    type BatchSenderCache = MutexOf<
        RuntimeOf<Build>,
        BTreeMap<
            (
                TargetChainId<Build, Target>,
                CounterpartyChainId<Build, Target>,
                TargetClientId<Build, Target>,
                CounterpartyClientId<Build, Target>,
            ),
            <TargetChain<Build, Target> as HasMessageBatchSenderType<Error>>::MessageBatchSender,
        >,
    >;
}
