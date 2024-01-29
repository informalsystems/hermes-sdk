use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;

use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::driver::traits::types::relay_at::{HasRelayTypeAt, RelayTypeAt};
use crate::types::index::Twindex;

#[derive_component(RelaySetupComponent, RelaySetup<Setup>)]
#[async_trait]
pub trait CanSetupRelays<const A: usize, const B: usize>:
    HasRelayTypeAt<A, B> + HasRelayTypeAt<B, A> + HasErrorType
where
    ChainTypeAt<Self, A>: HasIbcChainTypes<ChainTypeAt<Self, B>>,
    ChainTypeAt<Self, B>: HasIbcChainTypes<ChainTypeAt<Self, A>>,
{
    async fn setup_relays(
        &self,
        index: Twindex<A, B>,
        chain_a: &ChainTypeAt<Self, A>,
        chain_b: &ChainTypeAt<Self, B>,
        client_id_a: &ClientIdOf<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
        client_id_b: &ClientIdOf<ChainTypeAt<Self, B>, ChainTypeAt<Self, A>>,
    ) -> Result<(RelayTypeAt<Self, A, B>, RelayTypeAt<Self, B, A>), Self::Error>;
}
