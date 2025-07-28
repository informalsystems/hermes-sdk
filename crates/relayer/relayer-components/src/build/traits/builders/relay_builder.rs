use core::marker::PhantomData;
use core::time::Duration;

use hermes_chain_components::traits::HasClientIdType;
use hermes_prelude::*;

use crate::chain::traits::HasChainIdType;
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
        refresh_rate_a_to_b: Option<Duration>,
        refresh_rate_b_to_a: Option<Duration>,
    ) -> Result<Self::Relay, Self::Error>;
}
