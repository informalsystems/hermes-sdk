use cgp::prelude::*;
use hermes_chain_components::traits::types::packets::ack::{
    AcknowledgementOf, HasAcknowledgementType,
};

use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::chains::{HasRelayChains, PacketOf};

#[cgp_component {
  provider: ReceivePacketRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayReceivePacket:
    HasRelayChains<DstChain: HasAcknowledgementType<Self::SrcChain>>
{
    async fn relay_receive_packet(
        &self,
        source_height: &HeightOf<Self::SrcChain>,
        packet: &PacketOf<Self>,
    ) -> Result<Option<AcknowledgementOf<Self::DstChain, Self::SrcChain>>, Self::Error>;
}
