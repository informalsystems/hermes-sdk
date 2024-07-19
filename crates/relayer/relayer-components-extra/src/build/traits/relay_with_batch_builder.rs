use cgp_core::error::ErrorOf;
use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::{HasChainId, HasChainIdType};
use hermes_relayer_components::multi::traits::chain_at::{ChainTypeAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::{ClientIdAt, HasRelayTypeAt, RelayTypeAt};
use hermes_relayer_components::multi::types::index::Twindex;

use crate::batch::traits::channel::{HasMessageBatchSenderType, HasMessageBatchSenderTypes};
use crate::batch::types::aliases::MessageBatchSender;

#[derive_component(RelayWithBatchBuilderComponent, RelayWithBatchBuilder<Build>)]
#[async_trait]
pub trait CanBuildRelayWithBatch<const SRC: usize, const DST: usize>:
    HasRelayTypeAt<SRC, DST, Relay: HasMessageBatchSenderTypes>
    + HasChainTypeAt<
        SRC,
        Chain: HasChainIdType + HasMessageBatchSenderType<ErrorOf<RelayTypeAt<Self, SRC, DST>>>,
    > + HasChainTypeAt<
        DST,
        Chain: HasChainIdType + HasMessageBatchSenderType<ErrorOf<RelayTypeAt<Self, SRC, DST>>>,
    > + HasErrorType
{
    async fn build_relay_with_batch(
        &self,
        index: Twindex<SRC, DST>,
        src_client_id: &ClientIdAt<Self, SRC, DST>,
        dst_client_id: &ClientIdAt<Self, DST, SRC>,
        src_chain: ChainTypeAt<Self, SRC>,
        dst_chain: ChainTypeAt<Self, DST>,
        src_batch_sender: MessageBatchSender<
            ChainTypeAt<Self, SRC>,
            ErrorOf<RelayTypeAt<Self, SRC, DST>>,
        >,
        dst_batch_sender: MessageBatchSender<
            ChainTypeAt<Self, DST>,
            ErrorOf<RelayTypeAt<Self, SRC, DST>>,
        >,
    ) -> Result<Self::Relay, Self::Error>;
}
