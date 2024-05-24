use cgp_core::prelude::*;

use crate::chain::traits::types::ack::{AcknowledgementOf, HasAcknowledgementType};
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::chains::HasRelayChains;

#[derive_component(AckPacketRelayerComponent, AckPacketRelayer<Relay>)]
#[async_trait]
pub trait CanRelayAckPacket: HasRelayChains
where
    Self::DstChain: HasAcknowledgementType<Self::SrcChain>,
{
    async fn relay_ack_packet(
        &self,
        destination_height: &HeightOf<Self::DstChain>,
        packet: &Self::Packet,
        ack: &AcknowledgementOf<Self::DstChain, Self::SrcChain>,
    ) -> Result<(), Self::Error>;
}
