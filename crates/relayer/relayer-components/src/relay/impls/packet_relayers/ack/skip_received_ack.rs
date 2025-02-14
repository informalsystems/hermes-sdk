use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::packet::fields::{
    HasPacketSequence, HasPacketSrcChannelId, HasPacketSrcPortId,
};
use hermes_chain_components::traits::queries::ack_is_received::CanQueryAckIsReceived;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::packets::ack::HasAcknowledgementType;

use crate::components::default::relay::AckPacketRelayerComponent;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::packet_relayers::ack_packet::AckPacketRelayer;

pub struct SkipReceivedAck<InRelayer>(pub PhantomData<InRelayer>);

#[cgp_provider(AckPacketRelayerComponent)]
impl<Relay, SrcChain, DstChain, InRelayer> AckPacketRelayer<Relay> for SkipReceivedAck<InRelayer>
where
    Relay:
        HasRelayChains<SrcChain = SrcChain, DstChain = DstChain> + CanRaiseError<SrcChain::Error>,
    DstChain: HasHeightType + HasAcknowledgementType<SrcChain>,
    SrcChain: CanQueryAckIsReceived<DstChain>
        + HasPacketSrcChannelId<DstChain>
        + HasPacketSrcPortId<DstChain>
        + HasPacketSequence<DstChain>,
    InRelayer: AckPacketRelayer<Relay>,
{
    async fn relay_ack_packet(
        relay: &Relay,
        destination_height: &DstChain::Height,
        packet: &SrcChain::OutgoingPacket,
        ack: &DstChain::Acknowledgement,
    ) -> Result<(), Relay::Error> {
        let ack_received = relay
            .src_chain()
            .query_ack_is_received(
                &SrcChain::packet_src_port_id(packet),
                &SrcChain::packet_src_channel_id(packet),
                &SrcChain::packet_sequence(packet),
            )
            .await
            .map_err(Relay::raise_error)?;

        if !ack_received {
            InRelayer::relay_ack_packet(relay, destination_height, packet, ack).await?;
        }

        Ok(())
    }
}
