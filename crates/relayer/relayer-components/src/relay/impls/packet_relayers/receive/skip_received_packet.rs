use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::packet::fields::{
    HasPacketDstChannelId, HasPacketDstPortId, HasPacketSequence,
};
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::queries::packet_acknowledgement::CanQueryPacketAcknowledgement;

use crate::chain::traits::queries::packet_is_received::CanQueryPacketIsReceived;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains, PacketOf};
use crate::relay::traits::packet_relayers::receive_packet::{
    ReceivePacketRelayer, ReceivePacketRelayerComponent,
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
    DstChain: CanQueryChainHeight
        + HasWriteAckEvent<SrcChain>
        + CanQueryPacketIsReceived<SrcChain>
        + CanQueryPacketAcknowledgement<SrcChain>,
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
            let query_height = dst_chain
                .query_chain_height()
                .await
                .map_err(Relay::raise_error)?;

            let ack = dst_chain
                .query_packet_acknowledgement(
                    &SrcChain::packet_dst_channel_id(packet),
                    &SrcChain::packet_dst_port_id(packet),
                    &SrcChain::packet_sequence(packet),
                    &query_height,
                )
                .await
                .map_err(Relay::raise_error)?;

            Ok(Some(ack))
        }
    }
}
