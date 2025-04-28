use core::marker::PhantomData;

use hermes_chain_components::traits::{
    HasAcknowledgementType, HasPacketDstChannelId, HasPacketDstPortId, HasPacketSequence,
};
use hermes_prelude::*;

use crate::chain::traits::CanQueryPacketIsReceived;
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::{
    CanRaiseRelayChainErrors, HasRelayChains, PacketOf, ReceivePacketRelayer,
    ReceivePacketRelayerComponent,
};

pub struct SkipReceivedPacket<Relayer> {
    pub phantom: PhantomData<Relayer>,
}

#[cgp_provider(ReceivePacketRelayerComponent)]
impl<Relay, Relayer, SrcChain, DstChain> ReceivePacketRelayer<Relay> for SkipReceivedPacket<Relayer>
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain> + CanRaiseRelayChainErrors,
    Relayer: ReceivePacketRelayer<Relay>,
    SrcChain: HasPacketDstChannelId<DstChain>
        + HasPacketDstPortId<DstChain>
        + HasPacketSequence<DstChain>
        + HasErrorType,
    DstChain: CanQueryPacketIsReceived<SrcChain> + HasAcknowledgementType<SrcChain>,
{
    async fn relay_receive_packet(
        relay: &Relay,
        source_height: &HeightOf<Relay::SrcChain>,
        packet: &PacketOf<Relay>,
    ) -> Result<Option<DstChain::Acknowledgement>, Relay::Error> {
        let dst_chain = relay.dst_chain();

        let is_packet_received = dst_chain
            .query_packet_is_received(
                &Relay::SrcChain::packet_dst_port_id(packet),
                &Relay::SrcChain::packet_dst_channel_id(packet),
                &Relay::SrcChain::packet_sequence(packet),
            )
            .await
            .map_err(Relay::raise_error)?;

        if !is_packet_received {
            Relayer::relay_receive_packet(relay, source_height, packet).await
        } else {
            Ok(None)
        }
    }
}
