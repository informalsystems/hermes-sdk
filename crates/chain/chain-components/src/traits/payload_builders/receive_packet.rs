use cgp::prelude::*;
use hermes_chain_type_components::traits::HasOutgoingPacketType;

use crate::traits::{HasClientStateType, HasHeightType, HasReceivePacketPayloadType};

#[cgp_component {
  provider: ReceivePacketPayloadBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildReceivePacketPayload<Counterparty>:
    HasReceivePacketPayloadType<Counterparty>
    + HasOutgoingPacketType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasHeightType
    + HasAsyncErrorType
{
    async fn build_receive_packet_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        packet: &Self::OutgoingPacket,
    ) -> Result<Self::ReceivePacketPayload, Self::Error>;
}
