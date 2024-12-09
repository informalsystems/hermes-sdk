use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;

use crate::traits::types::client_state::HasClientStateType;
use crate::traits::types::height::HasHeightType;
use crate::traits::types::packets::receive::HasReceivePacketPayloadType;

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
    + HasErrorType
{
    async fn build_receive_packet_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        packet: &Self::OutgoingPacket,
    ) -> Result<Self::ReceivePacketPayload, Self::Error>;
}
