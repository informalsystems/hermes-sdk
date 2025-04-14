use cgp::prelude::*;
use hermes_chain_type_components::traits::HasOutgoingPacketType;

use crate::traits::{
    HasAckPacketPayloadType, HasAcknowledgementType, HasClientStateType, HasHeightType,
};

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
    + HasAsyncErrorType
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
