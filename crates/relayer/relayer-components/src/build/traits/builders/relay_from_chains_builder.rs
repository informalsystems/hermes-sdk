use cgp::prelude::*;

use crate::multi::traits::chain_at::ChainAt;
use crate::multi::traits::relay_at::{ClientIdAt, HasBoundedRelayTypeAt};
use crate::multi::types::index::Twindex;

#[derive_component(RelayFromChainsBuilderComponent, RelayFromChainsBuilder<Build>)]
#[async_trait]
pub trait CanBuildRelayFromChains<const SRC: usize, const DST: usize>:
    HasBoundedRelayTypeAt<SRC, DST> + HasErrorType
{
    async fn build_relay_from_chains(
        &self,
        index: Twindex<SRC, DST>,
        src_client_id: &ClientIdAt<Self, SRC, DST>,
        dst_client_id: &ClientIdAt<Self, DST, SRC>,
        src_chain: ChainAt<Self, SRC>,
        dst_chain: ChainAt<Self, DST>,
    ) -> Result<Self::Relay, Self::Error>;
}
