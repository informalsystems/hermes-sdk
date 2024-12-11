use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;

#[async_trait]
pub trait CanFilterOutgoingPacket<Counterparty>:
    HasOutgoingPacketType<Counterparty> + HasErrorType
{
    async fn should_relay_outgoing_packet(
        &self,
        packet: &Self::OutgoingPacket,
    ) -> Result<bool, Self::Error>;
}
