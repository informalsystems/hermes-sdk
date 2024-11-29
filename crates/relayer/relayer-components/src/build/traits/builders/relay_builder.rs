use cgp::prelude::*;

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::multi::traits::chain_at::{ChainIdAt, HasChainTypeAt};
use crate::multi::traits::relay_at::{ClientIdAt, HasBoundedRelayTypeAt};
use crate::multi::types::index::Twindex;

#[derive_component(RelayBuilderComponent, RelayBuilder<Build>)]
#[async_trait]
pub trait CanBuildRelay<const SRC: usize, const DST: usize>:
    HasBoundedRelayTypeAt<SRC, DST>
    + HasChainTypeAt<SRC, Chain: HasChainIdType>
    + HasChainTypeAt<DST, Chain: HasChainIdType>
    + HasErrorType
{
    async fn build_relay(
        &self,
        index: Twindex<SRC, DST>,
        src_chain_id: &ChainIdAt<Self, SRC>,
        dst_chain_id: &ChainIdAt<Self, DST>,
        src_client_id: &ClientIdAt<Self, SRC, DST>,
        dst_client_id: &ClientIdAt<Self, DST, SRC>,
    ) -> Result<Self::Relay, Self::Error>;
}
