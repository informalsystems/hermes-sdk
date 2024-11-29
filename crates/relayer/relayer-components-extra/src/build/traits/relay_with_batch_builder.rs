use cgp::core::error::ErrorOf;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::{
    ClientIdAt, HasBoundedRelayTypeAt, RelayAt,
};
use hermes_relayer_components::multi::types::index::Twindex;

use crate::batch::traits::channel::{HasMessageBatchSenderType, HasMessageBatchSenderTypes};
use crate::batch::types::aliases::MessageBatchSender;

#[derive_component(RelayWithBatchBuilderComponent, RelayWithBatchBuilder<Build>)]
#[async_trait]
pub trait CanBuildRelayWithBatch<const SRC: usize, const DST: usize>:
    HasBoundedRelayTypeAt<SRC, DST, Relay: HasMessageBatchSenderTypes>
    + HasChainTypeAt<
        SRC,
        Chain: HasChainIdType + HasMessageBatchSenderType<ErrorOf<RelayAt<Self, SRC, DST>>>,
    > + HasChainTypeAt<
        DST,
        Chain: HasChainIdType + HasMessageBatchSenderType<ErrorOf<RelayAt<Self, SRC, DST>>>,
    > + HasErrorType
{
    async fn build_relay_with_batch(
        &self,
        index: Twindex<SRC, DST>,
        src_client_id: &ClientIdAt<Self, SRC, DST>,
        dst_client_id: &ClientIdAt<Self, DST, SRC>,
        src_chain: ChainAt<Self, SRC>,
        dst_chain: ChainAt<Self, DST>,
        src_batch_sender: MessageBatchSender<ChainAt<Self, SRC>, ErrorOf<RelayAt<Self, SRC, DST>>>,
        dst_batch_sender: MessageBatchSender<ChainAt<Self, DST>, ErrorOf<RelayAt<Self, SRC, DST>>>,
    ) -> Result<Self::Relay, Self::Error>;
}
