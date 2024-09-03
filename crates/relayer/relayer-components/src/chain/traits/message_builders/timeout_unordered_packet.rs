use cgp::prelude::*;

use crate::chain::traits::types::message::HasMessageType;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;

#[derive_component(TimeoutUnorderedPacketMessageBuilderComponent, TimeoutUnorderedPacketMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildTimeoutUnorderedPacketMessage<Counterparty>:
    HasMessageType + HasIbcPacketTypes<Counterparty> + HasErrorType
where
    Counterparty: HasTimeoutUnorderedPacketPayloadType<Self>,
{
    async fn build_timeout_unordered_packet_message(
        &self,
        packet: &Self::OutgoingPacket,
        payload: Counterparty::TimeoutUnorderedPacketPayload,
    ) -> Result<Self::Message, Self::Error>;
}
