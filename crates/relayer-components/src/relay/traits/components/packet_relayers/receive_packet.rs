use cgp_async::async_generic_trait;
use cgp_macros::derive_component;

use crate::chain::traits::types::ibc_events::write_ack::HasWriteAcknowledgementEvent;
use crate::chain::types::aliases::{Height, WriteAcknowledgementEvent};
use crate::relay::traits::chains::HasRelayChains;
use crate::std_prelude::*;

#[derive_component(ReceivePacketRelayerComponnent, ReceivePacketRelayer<Relay>)]
#[async_generic_trait]
pub trait CanRelayReceivePacket: HasRelayChains
where
    Self::DstChain: HasWriteAcknowledgementEvent<Self::SrcChain>,
{
    async fn relay_receive_packet(
        &self,
        source_height: &Height<Self::SrcChain>,
        packet: &Self::Packet,
    ) -> Result<Option<WriteAcknowledgementEvent<Self::DstChain, Self::SrcChain>>, Self::Error>;
}
