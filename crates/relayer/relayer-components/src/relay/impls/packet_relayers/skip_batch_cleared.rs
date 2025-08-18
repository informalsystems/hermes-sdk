use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;

use hermes_chain_components::traits::{
    CanQueryPacketIsCleared, HasAcknowledgementType, HasHeightType, HasOutgoingPacketType,
    HasPacketSequence, HasPacketSrcChannelId, HasPacketSrcPortId,
};
use hermes_prelude::*;

use crate::relay::traits::{
    BatchAckPacketsRelayer, BatchAckPacketsRelayerComponent, BatchPacketsRelayer,
    BatchPacketsRelayerComponent, BatchReceivePacketsRelayer, BatchReceivePacketsRelayerComponent,
    BatchTimeoutUnorderedPacketsRelayer, BatchTimeoutUnorderedPacketsRelayerComponent,
    HasRelayChains,
};

pub struct BatchSkipClearedPackets<InRelayer>(pub PhantomData<InRelayer>);

#[cgp_provider(BatchReceivePacketsRelayerComponent)]
impl<Relay, SrcChain, DstChain, InRelayer> BatchReceivePacketsRelayer<Relay>
    for BatchSkipClearedPackets<InRelayer>
where
    Relay:
        HasRelayChains<SrcChain = SrcChain, DstChain = DstChain> + CanRaiseError<SrcChain::Error>,
    SrcChain: HasHeightType
        + CanQueryPacketIsCleared<DstChain>
        + HasPacketSrcChannelId<DstChain>
        + HasPacketSrcPortId<DstChain>
        + HasPacketSequence<DstChain>,
    DstChain: HasAcknowledgementType<SrcChain>,
    InRelayer: BatchReceivePacketsRelayer<Relay>,
{
    async fn relay_receive_packets(
        relay: &Relay,
        source_height: &SrcChain::Height,
        packets: Vec<&SrcChain::OutgoingPacket>,
    ) -> Result<Vec<Option<DstChain::Acknowledgement>>, Relay::Error> {
        let mut filtered_packets = vec![];

        for packet in packets.iter() {
            let packet_is_cleared = relay
                .src_chain()
                .query_packet_is_cleared(
                    &SrcChain::packet_src_port_id(packet),
                    &SrcChain::packet_src_channel_id(packet),
                    &SrcChain::packet_sequence(packet),
                )
                .await
                .map_err(Relay::raise_error)?;

            if !packet_is_cleared {
                filtered_packets.push(*packet);
            }
        }
        InRelayer::relay_receive_packets(relay, source_height, filtered_packets).await
    }
}

#[cgp_provider(BatchAckPacketsRelayerComponent)]
impl<Relay, SrcChain, DstChain, InRelayer> BatchAckPacketsRelayer<Relay>
    for BatchSkipClearedPackets<InRelayer>
where
    Relay:
        HasRelayChains<SrcChain = SrcChain, DstChain = DstChain> + CanRaiseError<SrcChain::Error>,
    DstChain: HasHeightType + HasAcknowledgementType<SrcChain>,
    SrcChain: CanQueryPacketIsCleared<DstChain>
        + HasPacketSrcChannelId<DstChain>
        + HasPacketSrcPortId<DstChain>
        + HasPacketSequence<DstChain>,
    InRelayer: BatchAckPacketsRelayer<Relay>,
{
    async fn relay_ack_packets(
        relay: &Relay,
        destination_heights: Vec<&DstChain::Height>,
        packets: Vec<&SrcChain::OutgoingPacket>,
        acks: Vec<&DstChain::Acknowledgement>,
    ) -> Result<(), Relay::Error> {
        let mut filtered_packets = vec![];
        let mut filtered_heights = vec![];
        let mut filtered_acks = vec![];

        for ((destination_height, packet), ack) in destination_heights
            .iter()
            .zip(packets.iter())
            .zip(acks.iter())
        {
            let packet_is_cleared = relay
                .src_chain()
                .query_packet_is_cleared(
                    &SrcChain::packet_src_port_id(packet),
                    &SrcChain::packet_src_channel_id(packet),
                    &SrcChain::packet_sequence(packet),
                )
                .await
                .map_err(Relay::raise_error)?;

            if !packet_is_cleared {
                filtered_packets.push(*packet);
                filtered_heights.push(*destination_height);
                filtered_acks.push(*ack);
            }
        }
        InRelayer::relay_ack_packets(relay, filtered_heights, filtered_packets, filtered_acks)
            .await?;

        Ok(())
    }
}

#[cgp_provider(BatchTimeoutUnorderedPacketsRelayerComponent)]
impl<Relay, SrcChain, DstChain, InRelayer> BatchTimeoutUnorderedPacketsRelayer<Relay>
    for BatchSkipClearedPackets<InRelayer>
where
    Relay:
        HasRelayChains<SrcChain = SrcChain, DstChain = DstChain> + CanRaiseError<SrcChain::Error>,
    SrcChain: HasOutgoingPacketType<DstChain>
        + CanQueryPacketIsCleared<DstChain>
        + HasPacketSrcChannelId<DstChain>
        + HasPacketSrcPortId<DstChain>
        + HasPacketSequence<DstChain>,
    DstChain: HasHeightType,
    InRelayer: BatchTimeoutUnorderedPacketsRelayer<Relay>,
{
    async fn relay_timeout_unordered_packets(
        relay: &Relay,
        destination_heights: Vec<&DstChain::Height>,
        packets: Vec<&SrcChain::OutgoingPacket>,
    ) -> Result<(), Relay::Error> {
        let mut filtered_packets = vec![];
        let mut filtered_heights = vec![];

        for (destination_height, packet) in destination_heights.iter().zip(packets.iter()) {
            let packet_is_cleared = relay
                .src_chain()
                .query_packet_is_cleared(
                    &SrcChain::packet_src_port_id(packet),
                    &SrcChain::packet_src_channel_id(packet),
                    &SrcChain::packet_sequence(packet),
                )
                .await
                .map_err(Relay::raise_error)?;

            if !packet_is_cleared {
                filtered_packets.push(*packet);
                filtered_heights.push(*destination_height);
            }
        }
        InRelayer::relay_timeout_unordered_packets(relay, filtered_heights, filtered_packets)
            .await?;

        Ok(())
    }
}

#[cgp_provider(BatchPacketsRelayerComponent)]
impl<Relay, SrcChain, DstChain, InRelayer> BatchPacketsRelayer<Relay>
    for BatchSkipClearedPackets<InRelayer>
where
    Relay:
        HasRelayChains<SrcChain = SrcChain, DstChain = DstChain> + CanRaiseError<SrcChain::Error>,
    SrcChain: HasOutgoingPacketType<DstChain>
        + CanQueryPacketIsCleared<DstChain>
        + HasPacketSrcChannelId<DstChain>
        + HasPacketSrcPortId<DstChain>
        + HasPacketSequence<DstChain>,
    InRelayer: BatchPacketsRelayer<Relay>,
{
    async fn relay_packets(
        relay: &Relay,
        packets: Vec<&SrcChain::OutgoingPacket>,
    ) -> Result<(), Relay::Error> {
        let mut filtered_packets = vec![];

        for packet in packets.iter() {
            let packet_is_cleared = relay
                .src_chain()
                .query_packet_is_cleared(
                    &SrcChain::packet_src_port_id(packet),
                    &SrcChain::packet_src_channel_id(packet),
                    &SrcChain::packet_sequence(packet),
                )
                .await
                .map_err(Relay::raise_error)?;

            if !packet_is_cleared {
                filtered_packets.push(*packet);
            }
        }
        InRelayer::relay_packets(relay, filtered_packets).await?;

        Ok(())
    }
}
