use cgp_core::prelude::*;

use crate::driver::traits::types::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use crate::driver::traits::types::chain_at::ChainTypeAt;

#[async_trait]
pub trait CanSetupBiRelay<const A: usize, const B: usize>:
    HasBiRelayTypeAt<A, B> + HasErrorType
{
    async fn setup_birelay(
        &self,
        chain_a: &ChainTypeAt<Self, A>,
        chain_b: &ChainTypeAt<Self, B>,
    ) -> Result<BiRelayTypeAt<Self, A, B>, Self::Error>;
}
