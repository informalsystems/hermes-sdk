use cgp::prelude::*;

use crate::traits::types::message::HasMessageType;
use crate::traits::types::packet::HasIbcPacketTypes;
use crate::traits::types::packets::receive::HasReceivePacketPayloadType;

#[derive_component(ReceivePacketMessageBuilderComponent, ReceivePacketMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildReceivePacketMessage<Counterparty>:
    HasMessageType + HasIbcPacketTypes<Counterparty> + HasErrorType
where
    Counterparty: HasReceivePacketPayloadType<Self>,
{
    async fn build_receive_packet_message(
        &self,
        packet: &Self::IncomingPacket,
        payload: Counterparty::ReceivePacketPayload,
    ) -> Result<Self::Message, Self::Error>;
}
