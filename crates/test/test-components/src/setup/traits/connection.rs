use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ConnectionIdOf;

use crate::driver::traits::types::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use crate::driver::traits::types::chain_at::ChainTypeAt;

#[derive_component(ConnectionSetupComponent, ConnectionSetup<Setup>)]
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
            ConnectionIdOf<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
            ConnectionIdOf<ChainTypeAt<Self, B>, ChainTypeAt<Self, A>>,
        ),
        Self::Error,
    >;
}
