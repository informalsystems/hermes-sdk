use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;

use crate::traits::types::message::HasMessageType;
use crate::traits::types::packets::receive::HasReceivePacketPayloadType;

#[cgp_component {
  name: ReceivePacketMessageBuilderComponent,
  provider: ReceivePacketMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildReceivePacketMessage<Counterparty>: HasMessageType + HasErrorType
where
    Counterparty: HasOutgoingPacketType<Self> + HasReceivePacketPayloadType<Self>,
{
    async fn build_receive_packet_message(
        &self,
        packet: &Counterparty::OutgoingPacket,
        payload: Counterparty::ReceivePacketPayload,
    ) -> Result<Self::Message, Self::Error>;
}
