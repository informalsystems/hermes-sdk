use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::types::packet::header::ProvidePacketHeaderType;
use crate::traits::types::packet::timeout::HasPacketTimeoutType;

#[derive(HasField)]
pub struct IbcPacketHeader<Chain, Counterparty>
where
    Chain: HasChannelIdType<Counterparty>,
    Counterparty: HasChannelIdType<Chain> + HasPacketTimeoutType<Chain>,
{
    pub src_channel_id: Chain::ChannelId,
    pub dst_channel_id: Counterparty::ChannelId,
    pub timeout: Counterparty::PacketTimeout,
}

pub struct UseIbcPacketHeader;

impl<Chain, Counterparty> ProvidePacketHeaderType<Chain, Counterparty> for UseIbcPacketHeader
where
    Chain: HasChannelIdType<Counterparty>,
    Counterparty: HasChannelIdType<Chain> + HasPacketTimeoutType<Chain>,
{
    type PacketHeader = IbcPacketHeader<Chain, Counterparty>;
}

impl<Chain, Counterparty> Clone for IbcPacketHeader<Chain, Counterparty>
where
    Chain: HasChannelIdType<Counterparty, ChannelId: Clone>,
    Counterparty: HasChannelIdType<Chain, ChannelId: Clone>
        + HasPacketTimeoutType<Chain, PacketTimeout: Clone>,
{
    fn clone(&self) -> Self {
        Self {
            src_channel_id: self.src_channel_id.clone(),
            dst_channel_id: self.dst_channel_id.clone(),
            timeout: self.timeout.clone(),
        }
    }
}
