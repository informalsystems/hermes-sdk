use hermes_chain_type_components::traits::HasChannelIdType;
use hermes_prelude::*;

use crate::traits::types::packet::header::{PacketHeaderTypeComponent, ProvidePacketHeaderType};
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

#[cgp_provider(PacketHeaderTypeComponent)]
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

impl<Chain, Counterparty> PartialEq for IbcPacketHeader<Chain, Counterparty>
where
    Chain: HasChannelIdType<Counterparty, ChannelId: Eq>,
    Counterparty:
        HasChannelIdType<Chain, ChannelId: Eq> + HasPacketTimeoutType<Chain, PacketTimeout: Eq>,
{
    fn eq(&self, other: &Self) -> bool {
        self.src_channel_id == other.src_channel_id
            && self.dst_channel_id == other.dst_channel_id
            && self.timeout == other.timeout
    }
}

impl<Chain, Counterparty> Eq for IbcPacketHeader<Chain, Counterparty>
where
    Chain: HasChannelIdType<Counterparty, ChannelId: Eq>,
    Counterparty:
        HasChannelIdType<Chain, ChannelId: Eq> + HasPacketTimeoutType<Chain, PacketTimeout: Eq>,
{
}
