use cgp_core::prelude::*;

use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::chain::traits::types::packets::ack::HasAckPacketPayload;
use crate::std_prelude::*;

#[derive_component(AckPacketPayloadBuilderComponent, AckPacketPayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildAckPacketPayload<Counterparty>:
    HasAckPacketPayload<Counterparty>
    + HasWriteAckEvent<Counterparty>
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
        ack: &Self::WriteAckEvent,
    ) -> Result<Self::AckPacketPayload, Self::Error>;
}
