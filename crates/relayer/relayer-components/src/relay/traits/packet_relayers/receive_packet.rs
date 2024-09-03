use cgp::prelude::*;

use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::types::aliases::{HeightOf, WriteAckEventOf};
use crate::relay::traits::chains::HasRelayChains;

#[derive_component(ReceivePacketRelayerComponent, ReceivePacketRelayer<Relay>)]
#[async_trait]
pub trait CanRelayReceivePacket:
    HasRelayChains<DstChain: HasWriteAckEvent<Self::SrcChain>>
{
    async fn relay_receive_packet(
        &self,
        source_height: &HeightOf<Self::SrcChain>,
        packet: &Self::Packet,
    ) -> Result<Option<WriteAckEventOf<Self::DstChain, Self::SrcChain>>, Self::Error>;
}
