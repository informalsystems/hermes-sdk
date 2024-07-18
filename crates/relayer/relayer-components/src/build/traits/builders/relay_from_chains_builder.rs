use cgp_core::prelude::*;

use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::chain_at::ChainTypeAt;
use crate::multi::traits::relay_at::HasRelayTypeAt;
use crate::multi::types::index::Twindex;

#[derive_component(RelayFromChainsBuilderComponent, RelayFromChainsBuilder<Build>)]
#[async_trait]
pub trait CanBuildRelayFromChains<const SRC: usize, const DST: usize>:
    HasRelayTypeAt<SRC, DST> + HasErrorType
{
    async fn build_relay_from_chains(
        &self,
        index: Twindex<SRC, DST>,
        src_client_id: &ClientIdOf<ChainTypeAt<Self, SRC>, ChainTypeAt<Self, DST>>,
        dst_client_id: &ClientIdOf<ChainTypeAt<Self, DST>, ChainTypeAt<Self, SRC>>,
        src_chain: ChainTypeAt<Self, SRC>,
        dst_chain: ChainTypeAt<Self, DST>,
    ) -> Result<Self::Relay, Self::Error>;
}
