use alloc::collections::BTreeMap;
use core::marker::PhantomData;

use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, ChainIdAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::ClientIdAt;
use hermes_runtime_components::traits::mutex::{HasMutex, MutexOf};
use hermes_runtime_components::traits::runtime::{HasRuntime, RuntimeOf};

use crate::batch::traits::channel::HasMessageBatchSenderType;
use crate::batch::types::aliases::MessageBatchSender;

pub trait HasBatchSenderCache<Error: Async, Target: Async, Counterparty: Async>:
    HasRuntime<Runtime: HasMutex>
    + HasChainTypeAt<
        Target,
        Chain: HasIbcChainTypes<ChainAt<Self, Counterparty>> + HasMessageBatchSenderType<Error>,
    > + HasChainTypeAt<Counterparty, Chain: HasIbcChainTypes<ChainAt<Self, Target>>>
{
    fn batch_sender_cache(
        &self,
        _index: PhantomData<(Target, Counterparty)>,
    ) -> &BatchSenderCache<Self, Error, Target, Counterparty>;
}

pub type BatchSenderCache<Build, Error, Target, Counterparty> = MutexOf<
    RuntimeOf<Build>,
    BTreeMap<
        (
            ChainIdAt<Build, Target>,
            ChainIdAt<Build, Counterparty>,
            ClientIdAt<Build, Target, Counterparty>,
            ClientIdAt<Build, Counterparty, Target>,
        ),
        MessageBatchSender<ChainAt<Build, Target>, Error>,
    >,
>;
