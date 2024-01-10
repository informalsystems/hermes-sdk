use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientId;

use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};

#[async_trait]
pub trait CanSetupClients<const A: usize, const B: usize>:
    HasChainTypeAt<A> + HasChainTypeAt<B> + HasErrorType
where
    ChainTypeAt<Self, A>: HasIbcChainTypes<ChainTypeAt<Self, B>>,
    ChainTypeAt<Self, B>: HasIbcChainTypes<ChainTypeAt<Self, A>>,
{
    async fn setup_birelay(
        &self,
        chain_a: &ChainTypeAt<Self, A>,
        chain_b: &ChainTypeAt<Self, B>,
    ) -> Result<
        (
            ClientId<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
            ClientId<ChainTypeAt<Self, B>, ChainTypeAt<Self, A>>,
        ),
        Self::Error,
    >;
}
