use cgp_core::prelude::*;

use crate::multi::traits::chain_at::ChainTypeAt;
use crate::multi::traits::relay_at::ClientIdAt;
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
        src_client_id: &ClientIdAt<Self, SRC, DST>,
        dst_client_id: &ClientIdAt<Self, DST, SRC>,
        src_chain: ChainTypeAt<Self, SRC>,
        dst_chain: ChainTypeAt<Self, DST>,
    ) -> Result<Self::Relay, Self::Error>;
}
