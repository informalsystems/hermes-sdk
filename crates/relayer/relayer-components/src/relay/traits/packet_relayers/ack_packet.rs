use cgp_core::prelude::*;

use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::types::aliases::{HeightOf, WriteAckEventOf};
use crate::relay::traits::chains::HasRelayChains;

#[derive_component(AckPacketRelayerComponent, AckPacketRelayer<Relay>)]
#[async_trait]
pub trait CanRelayAckPacket: HasRelayChains
where
    Self::DstChain: HasWriteAckEvent<Self::SrcChain>,
{
    async fn relay_ack_packet(
        &self,
        destination_height: &HeightOf<Self::DstChain>,
        packet: &Self::Packet,
        ack: &WriteAckEventOf<Self::DstChain, Self::SrcChain>,
    ) -> Result<(), Self::Error>;
}
