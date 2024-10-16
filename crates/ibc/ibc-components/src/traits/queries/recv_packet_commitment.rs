use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::types::packet::nonce::HasPacketNonceType;

#[derive_component(HasPacketReceivedQuerierComponent, HashPacketReceivedQuerier<Chain>)]
#[async_trait]
pub trait CanQueryHasPacketReceived<Counterparty>:
    HasChannelIdType<Counterparty> + HasErrorType
where
    Counterparty: HasChannelIdType<Self> + HasPacketNonceType<Self>,
{
    async fn query_has_packet_received(
        &self,
        src_channel_id: &Counterparty::ChannelId,
        dst_channel_id: &Self::ChannelId,
        nonce: &Counterparty::PacketNonce,
    ) -> Result<bool, Self::Error>;
}