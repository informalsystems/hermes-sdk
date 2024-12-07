use cgp::prelude::*;

use crate::chain::traits::types::packets::ack::{AcknowledgementOf, HasAcknowledgementType};
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::chains::{HasRelayChains, PacketOf};

#[cgp_component {
  name: AckPacketRelayerComponent,
  provider: AckPacketRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayAckPacket:
    HasRelayChains<DstChain: HasAcknowledgementType<Self::SrcChain>>
{
    async fn relay_ack_packet(
        &self,
        destination_height: &HeightOf<Self::DstChain>,
        packet: &PacketOf<Self>,
        ack: &AcknowledgementOf<Self::DstChain, Self::SrcChain>,
    ) -> Result<(), Self::Error>;
}
