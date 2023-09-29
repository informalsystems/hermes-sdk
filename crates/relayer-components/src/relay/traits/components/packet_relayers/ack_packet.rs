use cgp_async::async_generic_trait;
use cgp_macros::derive_component;

use crate::chain::traits::types::ibc_events::write_ack::HasWriteAcknowledgementEvent;
use crate::chain::types::aliases::{Height, WriteAcknowledgementEvent};
use crate::relay::traits::chains::HasRelayChains;
use crate::std_prelude::*;

#[derive_component(AckPacketRelayerComponent, AckPacketRelayer<Relay>)]
#[async_generic_trait]
pub trait CanRelayAckPacket: HasRelayChains
where
    Self::DstChain: HasWriteAcknowledgementEvent<Self::SrcChain>,
{
    async fn relay_ack_packet(
        &self,
        destination_height: &Height<Self::DstChain>,
        packet: &Self::Packet,
        ack: &WriteAcknowledgementEvent<Self::DstChain, Self::SrcChain>,
    ) -> Result<(), Self::Error>;
}
