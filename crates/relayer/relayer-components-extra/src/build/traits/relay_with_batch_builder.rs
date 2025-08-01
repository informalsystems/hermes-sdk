use core::marker::PhantomData;
use core::time::Duration;

use hermes_prelude::*;
use hermes_relayer_components::chain::traits::HasClientIdType;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::{ClientIdAt, HasRelayTypeAt};
use hermes_relayer_components::multi::types::tags::{Dst, Src};

use crate::batch::traits::types::{HasMessageBatchChannelTypes, MessageBatchSenderOf};

#[cgp_component {
  provider: RelayWithBatchBuilder,
  context: Build,
}]
#[async_trait]
pub trait CanBuildRelayWithBatch<A: Async, B: Async>: HasAsyncErrorType
    + HasChainTypeAt<A, Chain: HasClientIdType<ChainAt<Self, B>>>
    + HasChainTypeAt<B, Chain: HasClientIdType<ChainAt<Self, A>>>
    + HasRelayTypeAt<A, B, Relay: HasMessageBatchChannelTypes<Src> + HasMessageBatchChannelTypes<Dst>>
{
    async fn build_relay_with_batch(
        &self,
        _index: PhantomData<(A, B)>,
        src_client_id: &ClientIdAt<Self, A, B>,
        dst_client_id: &ClientIdAt<Self, B, A>,
        src_chain: ChainAt<Self, A>,
        dst_chain: ChainAt<Self, B>,
        src_batch_sender: MessageBatchSenderOf<Self::Relay, Src>,
        dst_batch_sender: MessageBatchSenderOf<Self::Relay, Dst>,
        refresh_rate_a: Option<Duration>,
        refresh_rate_b: Option<Duration>,
    ) -> Result<Self::Relay, Self::Error>;
}
