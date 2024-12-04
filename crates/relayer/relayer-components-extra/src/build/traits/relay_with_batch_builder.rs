use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::{
    ClientIdAt, HasBoundedRelayTypeAt, RelayAt,
};

use crate::batch::traits::channel::{HasMessageBatchSenderType, HasMessageBatchSenderTypes};
use crate::batch::types::aliases::MessageBatchSender;

#[derive_component(RelayWithBatchBuilderComponent, RelayWithBatchBuilder<Build>)]
#[async_trait]
pub trait CanBuildRelayWithBatch<Src: Async, Dst: Async>:
    HasBoundedRelayTypeAt<Src, Dst, Relay: HasMessageBatchSenderTypes>
    + HasChainTypeAt<
        Src,
        Chain: HasChainIdType + HasMessageBatchSenderType<ErrorOf<RelayAt<Self, Src, Dst>>>,
    > + HasChainTypeAt<
        Dst,
        Chain: HasChainIdType + HasMessageBatchSenderType<ErrorOf<RelayAt<Self, Src, Dst>>>,
    > + HasErrorType
{
    async fn build_relay_with_batch(
        &self,
        _index: PhantomData<(Src, Dst)>,
        src_client_id: &ClientIdAt<Self, Src, Dst>,
        dst_client_id: &ClientIdAt<Self, Dst, Src>,
        src_chain: ChainAt<Self, Src>,
        dst_chain: ChainAt<Self, Dst>,
        src_batch_sender: MessageBatchSender<ChainAt<Self, Src>, ErrorOf<RelayAt<Self, Src, Dst>>>,
        dst_batch_sender: MessageBatchSender<ChainAt<Self, Dst>, ErrorOf<RelayAt<Self, Src, Dst>>>,
    ) -> Result<Self::Relay, Self::Error>;
}
