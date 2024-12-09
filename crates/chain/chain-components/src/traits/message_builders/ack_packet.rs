use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;

use crate::traits::types::message::HasMessageType;
use crate::traits::types::packets::ack::HasAckPacketPayloadType;

#[cgp_component {
  provider: AckPacketMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildAckPacketMessage<Counterparty>:
    HasMessageType + HasOutgoingPacketType<Counterparty> + HasErrorType
where
    Counterparty: HasAckPacketPayloadType<Self>,
{
    async fn build_ack_packet_message(
        &self,
        packet: &Self::OutgoingPacket,
        payload: Counterparty::AckPacketPayload,
    ) -> Result<Self::Message, Self::Error>;
}
