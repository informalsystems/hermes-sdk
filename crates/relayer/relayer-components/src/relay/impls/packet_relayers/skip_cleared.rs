use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::packet::fields::{
    HasPacketSequence, HasPacketSrcChannelId, HasPacketSrcPortId,
};
use hermes_chain_components::traits::queries::packet_is_cleared::CanQueryPacketIsCleared;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_chain_components::traits::types::packets::ack::HasAcknowledgementType;

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::packet_relayer::{PacketRelayer, PacketRelayerComponent};
use crate::relay::traits::packet_relayers::ack_packet::{
    AckPacketRelayer, AckPacketRelayerComponent,
};
use crate::relay::traits::packet_relayers::receive_packet::{
    ReceivePacketRelayer, ReceivePacketRelayerComponent,
};
use crate::relay::traits::packet_relayers::timeout_unordered_packet::{
    TimeoutUnorderedPacketRelayer, TimeoutUnorderedPacketRelayerComponent,
};

pub struct SkipClearedPacket<InRelayer>(pub PhantomData<InRelayer>);

#[cgp_provider(ReceivePacketRelayerComponent)]
impl<Relay, SrcChain, DstChain, InRelayer> ReceivePacketRelayer<Relay>
    for SkipClearedPacket<InRelayer>
where
    Relay:
        HasRelayChains<SrcChain = SrcChain, DstChain = DstChain> + CanRaiseError<SrcChain::Error>,
    SrcChain: HasHeightType
        + CanQueryPacketIsCleared<DstChain>
        + HasPacketSrcChannelId<DstChain>
        + HasPacketSrcPortId<DstChain>
        + HasPacketSequence<DstChain>,
    DstChain: HasAcknowledgementType<SrcChain>,
    InRelayer: ReceivePacketRelayer<Relay>,
{
    async fn relay_receive_packet(
        relay: &Relay,
        source_height: &SrcChain::Height,
        packet: &SrcChain::OutgoingPacket,
    ) -> Result<Option<DstChain::Acknowledgement>, Relay::Error> {
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
            InRelayer::relay_receive_packet(relay, source_height, packet).await
        } else {
            Ok(None)
        }
    }
}

#[cgp_provider(AckPacketRelayerComponent)]
impl<Relay, SrcChain, DstChain, InRelayer> AckPacketRelayer<Relay> for SkipClearedPacket<InRelayer>
where
    Relay:
        HasRelayChains<SrcChain = SrcChain, DstChain = DstChain> + CanRaiseError<SrcChain::Error>,
    DstChain: HasHeightType + HasAcknowledgementType<SrcChain>,
    SrcChain: CanQueryPacketIsCleared<DstChain>
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
            InRelayer::relay_ack_packet(relay, destination_height, packet, ack).await?;
        }

        Ok(())
    }
}

#[cgp_provider(TimeoutUnorderedPacketRelayerComponent)]
impl<Relay, SrcChain, DstChain, InRelayer> TimeoutUnorderedPacketRelayer<Relay>
    for SkipClearedPacket<InRelayer>
where
    Relay:
        HasRelayChains<SrcChain = SrcChain, DstChain = DstChain> + CanRaiseError<SrcChain::Error>,
    SrcChain: HasOutgoingPacketType<DstChain>
        + CanQueryPacketIsCleared<DstChain>
        + HasPacketSrcChannelId<DstChain>
        + HasPacketSrcPortId<DstChain>
        + HasPacketSequence<DstChain>,
    DstChain: HasHeightType,
    InRelayer: TimeoutUnorderedPacketRelayer<Relay>,
{
    async fn relay_timeout_unordered_packet(
        relay: &Relay,
        destination_height: &DstChain::Height,
        packet: &SrcChain::OutgoingPacket,
    ) -> Result<(), Relay::Error> {
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
            InRelayer::relay_timeout_unordered_packet(relay, destination_height, packet).await?;
        }

        Ok(())
    }
}

#[cgp_provider(PacketRelayerComponent)]
impl<Relay, SrcChain, DstChain, InRelayer> PacketRelayer<Relay> for SkipClearedPacket<InRelayer>
where
    Relay:
        HasRelayChains<SrcChain = SrcChain, DstChain = DstChain> + CanRaiseError<SrcChain::Error>,
    SrcChain: HasOutgoingPacketType<DstChain>
        + CanQueryPacketIsCleared<DstChain>
        + HasPacketSrcChannelId<DstChain>
        + HasPacketSrcPortId<DstChain>
        + HasPacketSequence<DstChain>,
    InRelayer: PacketRelayer<Relay>,
{
    async fn relay_packet(
        relay: &Relay,
        packet: &SrcChain::OutgoingPacket,
    ) -> Result<(), Relay::Error> {
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
            InRelayer::relay_packet(relay, packet).await?;
        }

        Ok(())
    }
}
