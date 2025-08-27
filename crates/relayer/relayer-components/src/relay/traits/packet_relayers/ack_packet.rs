use hermes_prelude::*;

use crate::chain::traits::{AcknowledgementOf, HasAcknowledgementType};
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::{HasRelayChains, PacketOf};

#[cgp_component {
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

#[cgp_component {
  provider: BatchAckPacketsRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayBatchAckPackets:
    HasRelayChains<DstChain: HasAcknowledgementType<Self::SrcChain>>
{
    async fn relay_ack_packets(
        &self,
        packets_information: &[(
            PacketOf<Self>,
            AcknowledgementOf<Self::DstChain, Self::SrcChain>,
        )],
        batch_latest_height: &HeightOf<Self::DstChain>,
    ) -> Result<(), Self::Error>;
}
