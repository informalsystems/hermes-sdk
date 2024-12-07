use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;

use crate::traits::types::message::HasMessageType;
use crate::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;

#[cgp_component {
  name: TimeoutUnorderedPacketMessageBuilderComponent,
  provider: TimeoutUnorderedPacketMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildTimeoutUnorderedPacketMessage<Counterparty>:
    HasMessageType + HasOutgoingPacketType<Counterparty> + HasErrorType
where
    Counterparty: HasTimeoutUnorderedPacketPayloadType<Self>,
{
    async fn build_timeout_unordered_packet_message(
        &self,
        packet: &Self::OutgoingPacket,
        payload: Counterparty::TimeoutUnorderedPacketPayload,
    ) -> Result<Self::Message, Self::Error>;
}
