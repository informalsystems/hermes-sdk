use cgp_core::prelude::*;
use hermes_relayer_components::multi::traits::chain_at::ChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::{ClientIdAt, HasRelayTypeAt};
use hermes_relayer_components::multi::types::index::Twindex;

use crate::batch::traits::channel::HasMessageBatchSenderTypes;

#[derive_component(RelayWithBatchBuilderComponent, RelayWithBatchBuilder<Build>)]
#[async_trait]
pub trait CanBuildRelayWithBatch<const SRC: usize, const DST: usize>:
    HasRelayTypeAt<SRC, DST, Relay: HasMessageBatchSenderTypes> + HasErrorType
{
    async fn build_relay_with_batch(
        &self,
        index: Twindex<SRC, DST>,
        src_client_id: &ClientIdAt<Self, SRC, DST>,
        dst_client_id: &ClientIdAt<Self, DST, SRC>,
        src_chain: ChainTypeAt<Self, SRC>,
        dst_chain: ChainTypeAt<Self, DST>,
        src_batch_sender: <Self::Relay as HasMessageBatchSenderTypes>::SrcMessageBatchSender,
        dst_batch_sender: <Self::Relay as HasMessageBatchSenderTypes>::DstMessageBatchSender,
    ) -> Result<Self::Relay, Self::Error>;
}
