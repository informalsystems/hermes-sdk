use cgp_core::prelude::*;
use cgp_core::HasErrorType;

use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::chain::traits::types::packets::receive::HasReceivePacketPayload;
use crate::std_prelude::*;

#[derive_component(ReceivePacketPayloadBuilderComponent, ReceivePacketPayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildReceivePacketPayload<Counterparty>:
    HasReceivePacketPayload<Counterparty>
    + HasIbcPacketTypes<Counterparty>
    + HasClientStateType<Counterparty>
    + HasHeightType
    + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    async fn build_receive_packet_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        packet: &Self::OutgoingPacket,
    ) -> Result<Self::ReceivePacketPayload, Self::Error>;
}
