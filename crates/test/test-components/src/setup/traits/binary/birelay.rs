use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientId;

use crate::driver::traits::types::birelay_at::HasBiRelayTypeAt;
use crate::driver::traits::types::chain_at::ChainTypeAt;

#[async_trait]
pub trait CanSetupBiRelay: HasBiRelayTypeAt<0, 1> + HasErrorType
where
    ChainTypeAt<Self, 0>: HasIbcChainTypes<ChainTypeAt<Self, 1>>,
    ChainTypeAt<Self, 1>: HasIbcChainTypes<ChainTypeAt<Self, 0>>,
{
    async fn setup_birelay(
        &self,
        chain_a: &ChainTypeAt<Self, 0>,
        chain_b: &ChainTypeAt<Self, 1>,
        client_id_a: &ClientId<ChainTypeAt<Self, 0>, ChainTypeAt<Self, 1>>,
        client_id_b: &ClientId<ChainTypeAt<Self, 1>, ChainTypeAt<Self, 0>>,
    ) -> Result<Self::BiRelay, Self::Error>;
}
