use cgp::prelude::*;

use crate::traits::types::message::HasMessageType;
use crate::traits::types::packet::HasIbcPacketTypes;
use crate::traits::types::packets::ack::HasAckPacketPayloadType;

#[derive_component(AckPacketMessageBuilderComponent, AckPacketMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildAckPacketMessage<Counterparty>:
    HasMessageType + HasIbcPacketTypes<Counterparty> + HasErrorType
where
    Counterparty: HasAckPacketPayloadType<Self>,
{
    async fn build_ack_packet_message(
        &self,
        packet: &Self::OutgoingPacket,
        payload: Counterparty::AckPacketPayload,
    ) -> Result<Self::Message, Self::Error>;
}
