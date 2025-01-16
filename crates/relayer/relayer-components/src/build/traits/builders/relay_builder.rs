use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::types::ibc::HasClientIdType;

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::multi::traits::chain_at::{ChainAt, ChainIdAt, HasChainTypeAt};
use crate::multi::traits::relay_at::{ClientIdAt, HasRelayTypeAt};

#[cgp_component {
  provider: RelayBuilder,
  context: Build,
}]
#[async_trait]
pub trait CanBuildRelay<Src: Async, Dst: Async>:
    HasRelayTypeAt<Src, Dst>
    + HasChainTypeAt<Src, Chain: HasChainIdType + HasClientIdType<ChainAt<Self, Dst>>>
    + HasChainTypeAt<Dst, Chain: HasChainIdType + HasClientIdType<ChainAt<Self, Src>>>
    + HasAsyncErrorType
{
    async fn build_relay(
        &self,
        _tag: PhantomData<(Src, Dst)>,
        src_chain_id: &ChainIdAt<Self, Src>,
        dst_chain_id: &ChainIdAt<Self, Dst>,
        src_client_id: &ClientIdAt<Self, Src, Dst>,
        dst_client_id: &ClientIdAt<Self, Dst, Src>,
    ) -> Result<Self::Relay, Self::Error>;
}
