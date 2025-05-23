use hermes_chain_type_components::traits::{HasIncomingPacketType, HasOutgoingPacketType};
use hermes_prelude::*;

#[cgp_component {
    context: Chain,
    provider: OutgoingPacketFilter,
}]
#[async_trait]
pub trait CanFilterOutgoingPacket<Counterparty>:
    HasOutgoingPacketType<Counterparty> + HasAsyncErrorType
{
    async fn should_relay_outgoing_packet(
        &self,
        packet: &Self::OutgoingPacket,
    ) -> Result<bool, Self::Error>;
}

#[cgp_component {
    context: Chain,
    provider: IncomingPacketFilter,
}]
#[async_trait]
pub trait CanFilterIncomingPacket<Counterparty>:
    HasIncomingPacketType<Counterparty> + HasAsyncErrorType
{
    async fn should_relay_incoming_packet(
        &self,
        packet: &Self::IncomingPacket,
    ) -> Result<bool, Self::Error>;
}
