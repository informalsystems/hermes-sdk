use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;
use hermes_chain_type_components::traits::types::ibc::sequence::HasSequenceType;

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
