use cgp_core::prelude::*;

use crate::chain::traits::types::message::HasMessageType;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::chain::traits::types::packets::ack::HasAckPacketPayload;
use crate::std_prelude::*;

#[derive_component(AckPacketMessageBuilderComponent, AckPacketMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildAckPacketMessage<Counterparty>:
    HasMessageType + HasErrorType + HasIbcPacketTypes<Counterparty>
where
    Counterparty: HasAckPacketPayload<Self>,
{
    async fn build_ack_packet_message(
        &self,
        packet: &Self::OutgoingPacket,
        payload: Counterparty::AckPacketPayload,
    ) -> Result<Self::Message, Self::Error>;
}
