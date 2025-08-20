use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;

use hermes_chain_components::traits::{
    HasAcknowledgementType, HasPacketDstChannelId, HasPacketDstPortId, HasPacketSequence,
};
use hermes_prelude::*;

use crate::chain::traits::CanQueryPacketIsReceived;
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::{
    BatchReceivePacketsRelayer, BatchReceivePacketsRelayerComponent, CanRaiseRelayChainErrors,
    HasRelayChains, PacketOf,
};

pub struct BatchSkipReceivedPackets<Relayer> {
    pub phantom: PhantomData<Relayer>,
}

#[cgp_provider(BatchReceivePacketsRelayerComponent)]
impl<Relay, Relayer, SrcChain, DstChain> BatchReceivePacketsRelayer<Relay>
    for BatchSkipReceivedPackets<Relayer>
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain> + CanRaiseRelayChainErrors,
    Relayer: BatchReceivePacketsRelayer<Relay>,
    SrcChain: HasPacketDstChannelId<DstChain>
        + HasPacketDstPortId<DstChain>
        + HasPacketSequence<DstChain>
        + HasErrorType,
    DstChain: CanQueryPacketIsReceived<SrcChain> + HasAcknowledgementType<SrcChain>,
{
    async fn relay_receive_packets(
        relay: &Relay,
        source_height: &HeightOf<Relay::SrcChain>,
        packets: Vec<&PacketOf<Relay>>,
    ) -> Result<Vec<Option<DstChain::Acknowledgement>>, Relay::Error> {
        if packets.is_empty() {
            return Ok(vec![]);
        }

        let dst_chain = relay.dst_chain();

        let mut filtered_packets = vec![];

        for packet in packets.iter() {
            let is_packet_received = dst_chain
                .query_packet_is_received(
                    &Relay::SrcChain::packet_dst_port_id(packet),
                    &Relay::SrcChain::packet_dst_channel_id(packet),
                    &Relay::SrcChain::packet_sequence(packet),
                )
                .await
                .map_err(Relay::raise_error)?;

            if !is_packet_received {
                filtered_packets.push(packet);
            }
        }
        Relayer::relay_receive_packets(relay, source_height, packets).await
    }
}
