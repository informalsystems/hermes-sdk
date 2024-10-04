use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::types::packet::nonce::HasPacketNonceType;

#[derive(HasField)]
pub struct PacketHeader<Chain, Counterparty>
where
    Chain: HasChannelIdType<Counterparty> + HasPacketNonceType<Counterparty>,
    Counterparty: HasChannelIdType<Chain>,
{
    pub src_channel_id: Chain::ChannelId,
    pub dst_channel_id: Counterparty::ChannelId,
    pub nonce: Chain::PacketNonce,
}
