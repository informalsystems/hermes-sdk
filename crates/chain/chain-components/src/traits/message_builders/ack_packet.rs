use hermes_chain_type_components::traits::HasOutgoingPacketType;
use hermes_prelude::*;

use crate::traits::{HasAckPacketPayloadType, HasMessageType};

#[cgp_component {
  provider: AckPacketMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildAckPacketMessage<Counterparty>:
    HasMessageType + HasOutgoingPacketType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasAckPacketPayloadType<Self>,
{
    async fn build_ack_packet_message(
        &self,
        packet: &Self::OutgoingPacket,
        payload: Counterparty::AckPacketPayload,
    ) -> Result<Self::Message, Self::Error>;
}
