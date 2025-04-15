use cgp::prelude::*;
use hermes_chain_type_components::traits::HasOutgoingPacketType;

use crate::traits::{HasMessageType, HasTimeoutUnorderedPacketPayloadType};

#[cgp_component {
  provider: TimeoutUnorderedPacketMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildTimeoutUnorderedPacketMessage<Counterparty>:
    HasMessageType + HasOutgoingPacketType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasTimeoutUnorderedPacketPayloadType<Self>,
{
    async fn build_timeout_unordered_packet_message(
        &self,
        packet: &Self::OutgoingPacket,
        payload: Counterparty::TimeoutUnorderedPacketPayload,
    ) -> Result<Self::Message, Self::Error>;
}
