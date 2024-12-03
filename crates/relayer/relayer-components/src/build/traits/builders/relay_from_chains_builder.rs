use core::marker::PhantomData;

use cgp::prelude::*;

use crate::multi::traits::chain_at::ChainAt;
use crate::multi::traits::relay_at::{ClientIdAt, HasBoundedRelayTypeAt};

#[derive_component(RelayFromChainsBuilderComponent, RelayFromChainsBuilder<Build>)]
#[async_trait]
pub trait CanBuildRelayFromChains<Src: Async, Dst: Async>:
    HasBoundedRelayTypeAt<Src, Dst> + HasErrorType
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
