use alloc::vec::Vec;
use cgp_core::prelude::*;

use crate::chain::traits::types::{channel::HasChannelEndsType, ibc::HasIbcChainTypes};

#[derive_component(ChannelQuerierComponent, ChannelQuerier<Chain>)]
#[async_trait]
pub trait CanQueryChannel<Counterparty>: HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasChannelEndsType<Self>,
{
    async fn query_channel(
        &self,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        height: &Self::Height,
    ) -> Result<Counterparty::ChannelEnd, Self::Error>;
}

#[derive_component(ChannelBytesQuerierComponent, ChannelBytesQuerier<Chain>)]
#[async_trait]
pub trait CanQueryChannelBytes<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
{
    async fn query_channel_bytes(
        &self,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        height: &Self::Height,
    ) -> Result<Vec<u8>, Self::Error>;
}

