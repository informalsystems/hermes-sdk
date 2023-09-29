use cgp_async::async_generic_trait;
use cgp_core::traits::HasErrorType;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::std_prelude::*;

#[async_generic_trait]
pub trait ReceivedPacketQuerier<Chain, Counterparty>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain>,
{
    async fn query_is_packet_received(
        chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        sequence: &Counterparty::Sequence,
    ) -> Result<bool, Chain::Error>;
}

#[async_generic_trait]
pub trait CanQueryReceivedPacket<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    async fn query_is_packet_received(
        &self,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        sequence: &Counterparty::Sequence,
    ) -> Result<bool, Self::Error>;
}
