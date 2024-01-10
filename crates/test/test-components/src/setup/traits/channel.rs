use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelId, ConnectionId};

use crate::driver::traits::types::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use crate::driver::traits::types::chain_at::ChainTypeAt;

#[derive_component(ChannelSetupComponent, ChannelSetup<Setup>)]
#[async_trait]
pub trait CanSetupChannel<const A: usize, const B: usize>:
    HasBiRelayTypeAt<A, B> + HasErrorType
where
    ChainTypeAt<Self, A>: HasIbcChainTypes<ChainTypeAt<Self, B>>,
    ChainTypeAt<Self, B>: HasIbcChainTypes<ChainTypeAt<Self, A>>,
{
    async fn setup_channel(
        &self,
        birelay: &BiRelayTypeAt<Self, A, B>,
        connection_id_a: &ConnectionId<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
        connection_id_b: &ConnectionId<ChainTypeAt<Self, B>, ChainTypeAt<Self, A>>,
    ) -> Result<
        (
            ChannelId<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
            ChannelId<ChainTypeAt<Self, B>, ChainTypeAt<Self, A>>,
        ),
        Self::Error,
    >;
}
