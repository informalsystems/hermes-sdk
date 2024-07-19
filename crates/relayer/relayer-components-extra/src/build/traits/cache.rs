use alloc::collections::BTreeMap;

use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::multi::traits::chain_at::{ChainIdAt, ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::ClientIdAt;
use hermes_relayer_components::multi::types::index::Twindex;
use hermes_runtime_components::traits::mutex::{HasMutex, MutexOf};
use hermes_runtime_components::traits::runtime::{HasRuntime, RuntimeOf};

use crate::batch::traits::channel::HasMessageBatchSenderType;
use crate::batch::types::aliases::MessageBatchSender;

pub trait HasBatchSenderCache<Error: Async, const TARGET: usize, const COUNTERPARTY: usize>:
    HasRuntime<Runtime: HasMutex>
    + HasChainTypeAt<
        TARGET,
        Chain: HasIbcChainTypes<ChainAt<Self, COUNTERPARTY>> + HasMessageBatchSenderType<Error>,
    > + HasChainTypeAt<COUNTERPARTY, Chain: HasIbcChainTypes<ChainAt<Self, TARGET>>>
{
    fn batch_sender_cache(
        &self,
        index: Twindex<TARGET, COUNTERPARTY>,
    ) -> &BatchSenderCache<Self, Error, TARGET, COUNTERPARTY>;
}

pub type BatchSenderCache<Build, Error, const TARGET: usize, const COUNTERPARTY: usize> = MutexOf<
    RuntimeOf<Build>,
    BTreeMap<
        (
            ChainIdAt<Build, TARGET>,
            ChainIdAt<Build, COUNTERPARTY>,
            ClientIdAt<Build, TARGET, COUNTERPARTY>,
            ClientIdAt<Build, COUNTERPARTY, TARGET>,
        ),
        MessageBatchSender<ChainAt<Build, TARGET>, Error>,
    >,
>;
