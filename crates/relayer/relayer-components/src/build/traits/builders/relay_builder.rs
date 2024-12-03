use core::marker::PhantomData;

use cgp::prelude::*;

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::multi::traits::chain_at::{ChainIdAt, HasChainTypeAt};
use crate::multi::traits::relay_at::{ClientIdAt, HasBoundedRelayTypeAt};

#[derive_component(RelayBuilderComponent, RelayBuilder<Build>)]
#[async_trait]
pub trait CanBuildRelay<Src: Async, Dst: Async>:
    HasBoundedRelayTypeAt<Src, Dst>
    + HasChainTypeAt<Src, Chain: HasChainIdType>
    + HasChainTypeAt<Dst, Chain: HasChainIdType>
    + HasErrorType
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
