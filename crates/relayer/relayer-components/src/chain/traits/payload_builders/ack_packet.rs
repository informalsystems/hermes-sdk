use cgp::prelude::*;

use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::chain::traits::types::packets::ack::{HasAckPacketPayloadType, HasAcknowledgementType};

#[derive_component(AckPacketPayloadBuilderComponent, AckPacketPayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildAckPacketPayload<Counterparty>:
    HasAckPacketPayloadType<Counterparty>
    + HasAcknowledgementType<Counterparty>
    + HasIbcPacketTypes<Counterparty>
    + HasClientStateType<Counterparty>
    + HasHeightType
    + HasErrorType
{
    async fn build_ack_packet_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        packet: &Self::IncomingPacket,
        ack: &Self::Acknowledgement,
    ) -> Result<Self::AckPacketPayload, Self::Error>;
}
