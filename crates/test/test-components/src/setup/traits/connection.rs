use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ConnectionId;

use crate::driver::traits::types::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use crate::driver::traits::types::chain_at::ChainTypeAt;

#[async_trait]
pub trait CanSetupConnection<const A: usize, const B: usize>:
    HasBiRelayTypeAt<A, B> + HasErrorType
where
    ChainTypeAt<Self, A>: HasIbcChainTypes<ChainTypeAt<Self, B>>,
    ChainTypeAt<Self, B>: HasIbcChainTypes<ChainTypeAt<Self, A>>,
{
    async fn setup_connection(
        &self,
        birelay: &BiRelayTypeAt<Self, A, B>,
    ) -> Result<
        (
            ConnectionId<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
            ConnectionId<ChainTypeAt<Self, B>, ChainTypeAt<Self, A>>,
        ),
        Self::Error,
    >;
}
