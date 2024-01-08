use cgp_core::prelude::*;

use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::types::aliases::{Height, WriteAckEvent};
use crate::relay::traits::chains::HasRelayChains;

#[derive_component(ReceivePacketRelayerComponnent, ReceivePacketRelayer<Relay>)]
#[async_trait]
pub trait CanRelayReceivePacket: HasRelayChains
where
    Self::DstChain: HasWriteAckEvent<Self::SrcChain>,
{
    async fn relay_receive_packet(
        &self,
        source_height: &Height<Self::SrcChain>,
        packet: &Self::Packet,
    ) -> Result<Option<WriteAckEvent<Self::DstChain, Self::SrcChain>>, Self::Error>;
}
