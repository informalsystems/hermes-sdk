use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;

use crate::traits::types::client_state::HasClientStateType;
use crate::traits::types::height::HasHeightType;
use crate::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;

#[derive_component(TimeoutUnorderedPacketPayloadBuilderComponent, TimeoutUnorderedPacketPayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildTimeoutUnorderedPacketPayload<Counterparty>:
    HasTimeoutUnorderedPacketPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasHeightType
    + HasErrorType
where
    Counterparty: HasOutgoingPacketType<Self>,
{
    async fn build_timeout_unordered_packet_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        packet: &Counterparty::OutgoingPacket,
    ) -> Result<Self::TimeoutUnorderedPacketPayload, Self::Error>;
}
