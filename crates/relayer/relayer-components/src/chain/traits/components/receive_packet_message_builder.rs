use cgp_core::prelude::*;

use crate::chain::traits::types::message::HasMessageType;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::chain::traits::types::packets::receive::HasReceivePacketPayload;
use crate::std_prelude::*;

#[derive_component(ReceivePacketMessageBuilderComponent, ReceivePacketMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildReceivePacketMessage<Counterparty>:
    HasMessageType + HasErrorType + HasIbcPacketTypes<Counterparty>
where
    Counterparty: HasReceivePacketPayload<Self>,
{
    async fn build_receive_packet_message(
        &self,
        packet: &Self::IncomingPacket,
        payload: Counterparty::ReceivePacketPayload,
    ) -> Result<Self::Message, Self::Error>;
}
