use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;

use crate::traits::types::client_state::HasClientStateType;
use crate::traits::types::height::HasHeightType;
use crate::traits::types::packets::ack::{HasAckPacketPayloadType, HasAcknowledgementType};

#[cgp_component {
  provider: AckPacketPayloadBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildAckPacketPayload<Counterparty>:
    HasAckPacketPayloadType<Counterparty>
    + HasAcknowledgementType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasHeightType
    + HasErrorType
where
    Counterparty: HasOutgoingPacketType<Self>,
{
    async fn build_ack_packet_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        packet: &Counterparty::OutgoingPacket,
        ack: &Self::Acknowledgement,
    ) -> Result<Self::AckPacketPayload, Self::Error>;
}
