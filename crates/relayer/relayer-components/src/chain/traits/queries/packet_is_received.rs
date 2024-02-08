use cgp_core::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;

#[derive_component(ReceivedPacketQuerierComponent, ReceivedPacketQuerier<Chain>)]
#[async_trait]
pub trait CanQueryPacketIsReceived<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    async fn query_packet_is_received(
        &self,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        sequence: &Counterparty::Sequence,
    ) -> Result<bool, Self::Error>;
}
