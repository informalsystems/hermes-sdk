use cgp_core::{async_trait, derive_component};

use crate::chain::traits::types::ibc_events::write_ack::HasWriteAcknowledgementEvent;
use crate::chain::types::aliases::{Height, WriteAcknowledgementEvent};
use crate::relay::traits::chains::HasRelayChains;
use crate::std_prelude::*;

#[derive_component(ReceivePacketRelayerComponnent, ReceivePacketRelayer<Relay>)]
#[async_trait]
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
