use hermes_chain_type_components::traits::{HasChannelIdType, HasPortIdType, HasSequenceType};
use hermes_prelude::*;

/**
   Checks if a given packet has been cleared on the source chain. Having
   the packet cleared means that the source chain has received either
   an ack or timeout from the destination chain, thereby completing
   the full packet relaying cycle.
*/
#[cgp_component {
    provider: PacketIsClearedQuerier,
    context: Chain,
}]
#[async_trait]
pub trait CanQueryPacketIsCleared<Counterparty>:
    HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasSequenceType<Counterparty>
    + HasAsyncErrorType
{
    async fn query_packet_is_cleared(
        &self,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        sequence: &Self::Sequence,
    ) -> Result<bool, Self::Error>;
}
