use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ChannelId;

use crate::driver::traits::types::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::setup::traits::driver::HasDriverType;

#[async_trait]
pub trait CanSetupDriverWithBinaryChannel: HasDriverType + HasErrorType
where
    Self::Driver: HasBiRelayTypeAt<0, 1>,
    ChainTypeAt<Self::Driver, 0>: HasIbcChainTypes<ChainTypeAt<Self::Driver, 1>>,
    ChainTypeAt<Self::Driver, 1>: HasIbcChainTypes<ChainTypeAt<Self::Driver, 0>>,
{
    async fn setup_driver_with_binary_channel(
        &self,
        birelay: BiRelayTypeAt<Self::Driver, 0, 1>,
        channel_id_a: ChannelId<ChainTypeAt<Self::Driver, 0>, ChainTypeAt<Self::Driver, 1>>,
        channel_id_b: ChannelId<ChainTypeAt<Self::Driver, 1>, ChainTypeAt<Self::Driver, 0>>,
    ) -> Result<Self::Driver, Self::Error>;
}
