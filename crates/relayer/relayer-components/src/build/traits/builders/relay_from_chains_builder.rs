use core::marker::PhantomData;
use core::time::Duration;

use hermes_chain_components::traits::HasClientIdType;
use hermes_prelude::*;

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
        refresh_rate_a_to_b: Option<Duration>,
        refresh_rate_b_to_a: Option<Duration>,
    ) -> Result<Self::Relay, Self::Error>;
}
