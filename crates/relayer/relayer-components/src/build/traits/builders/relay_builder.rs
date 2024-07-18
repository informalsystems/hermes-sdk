use cgp_core::prelude::*;

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::types::aliases::{ChainIdOf, ClientIdOf};
use crate::multi::traits::chain_at::ChainTypeAt;
use crate::multi::traits::chain_at::HasChainTypeAt;
use crate::multi::traits::relay_at::HasRelayTypeAt;
use crate::multi::types::index::Twindex;

#[derive_component(RelayBuilderComponent, RelayBuilder<Build>)]
#[async_trait]
pub trait CanBuildRelay<const SRC: usize, const DST: usize>:
    HasRelayTypeAt<SRC, DST>
    + HasChainTypeAt<SRC, Chain: HasChainIdType>
    + HasChainTypeAt<DST, Chain: HasChainIdType>
    + HasErrorType
{
    async fn build_relay(
        &self,
        index: Twindex<SRC, DST>,
        src_chain_id: &ChainIdOf<ChainTypeAt<Self, SRC>>,
        dst_chain_id: &ChainIdOf<ChainTypeAt<Self, DST>>,
        src_client_id: &ClientIdOf<ChainTypeAt<Self, SRC>, ChainTypeAt<Self, DST>>,
        dst_client_id: &ClientIdOf<ChainTypeAt<Self, DST>, ChainTypeAt<Self, SRC>>,
    ) -> Result<Self::Relay, Self::Error>;
}
