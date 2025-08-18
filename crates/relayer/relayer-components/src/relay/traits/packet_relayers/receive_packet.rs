use alloc::vec::Vec;

use hermes_chain_components::traits::{AcknowledgementOf, HasAcknowledgementType};
use hermes_prelude::*;

use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::{HasRelayChains, PacketOf};

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

#[cgp_component {
  provider: BatchReceivePacketsRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayBatchReceivePackets:
    HasRelayChains<DstChain: HasAcknowledgementType<Self::SrcChain>>
{
    async fn relay_receive_packets(
        &self,
        source_height: &HeightOf<Self::SrcChain>,
        packets: Vec<&PacketOf<Self>>,
    ) -> Result<Vec<Option<AcknowledgementOf<Self::DstChain, Self::SrcChain>>>, Self::Error>;
}
