use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::types::packet::header::ProvidePacketHeaderType;
use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::timeout::HasPacketTimeoutType;

#[derive(HasField)]
pub struct IbcPacketHeader<Chain, Counterparty>
where
    Chain: HasChannelIdType<Counterparty> + HasPacketNonceType<Counterparty>,
    Counterparty: HasChannelIdType<Chain> + HasPacketTimeoutType<Chain>,
{
    pub src_channel_id: Chain::ChannelId,
    pub dst_channel_id: Counterparty::ChannelId,
    pub nonce: Chain::PacketNonce,
    pub timeout: Counterparty::PacketTimeout,
}

pub struct UseIbcPacketHeader;

impl<Chain, Counterparty> ProvidePacketHeaderType<Chain, Counterparty> for UseIbcPacketHeader
where
    Chain: HasChannelIdType<Counterparty> + HasPacketNonceType<Counterparty>,
    Counterparty: HasChannelIdType<Chain> + HasPacketTimeoutType<Chain>,
{
    type PacketHeader = IbcPacketHeader<Chain, Counterparty>;
}
