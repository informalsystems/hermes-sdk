use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::types::ibc::HasClientIdType;

use crate::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use crate::multi::traits::relay_at::{ClientIdAt, HasRelayTypeAt};

#[cgp_component {
  provider: RelayFromChainsBuilder,
  context: Build,
}]
#[async_trait]
pub trait CanBuildRelayFromChains<Src: Async, Dst: Async>:
    HasRelayTypeAt<Src, Dst>
    + HasChainTypeAt<Src, Chain: HasClientIdType<ChainAt<Self, Dst>>>
    + HasChainTypeAt<Dst, Chain: HasClientIdType<ChainAt<Self, Src>>>
    + HasAsyncErrorType
{
    async fn build_relay_from_chains(
        &self,
        _tag: PhantomData<(Src, Dst)>,
        src_client_id: &ClientIdAt<Self, Src, Dst>,
        dst_client_id: &ClientIdAt<Self, Dst, Src>,
        src_chain: ChainAt<Self, Src>,
        dst_chain: ChainAt<Self, Dst>,
    ) -> Result<Self::Relay, Self::Error>;
}
