use cgp_core::prelude::*;

use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::message::HasMessageType;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayload;

#[derive_component(TimeoutUnorderedPacketPayloadBuilderComponent, TimeoutUnorderedPacketPayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildTimeoutUnorderedPacketPayload<Counterparty>:
    HasTimeoutUnorderedPacketPayload<Counterparty>
    + HasIbcPacketTypes<Counterparty>
    + HasClientStateType<Counterparty>
    + HasHeightType
    + HasErrorType
{
    async fn build_timeout_unordered_packet_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        packet: &Self::IncomingPacket,
    ) -> Result<Self::TimeoutUnorderedPacketPayload, Self::Error>;
}

#[derive_component(TimeoutUnorderedPacketMessageBuilderComponent, TimeoutUnorderedPacketMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildTimeoutUnorderedPacketMessage<Counterparty>:
    HasMessageType + HasErrorType + HasIbcPacketTypes<Counterparty>
where
    Counterparty: HasTimeoutUnorderedPacketPayload<Self>,
{
    async fn build_timeout_unordered_packet_message(
        &self,
        packet: &Self::OutgoingPacket,
        payload: Counterparty::TimeoutUnorderedPacketPayload,
    ) -> Result<Self::Message, Self::Error>;
}
