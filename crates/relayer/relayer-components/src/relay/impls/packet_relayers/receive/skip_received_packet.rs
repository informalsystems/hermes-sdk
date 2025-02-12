use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::packet::fields::{
    HasPacketDstChannelId, HasPacketDstPortId, HasPacketSequence,
};

use crate::chain::traits::queries::packet_is_received::CanQueryPacketIsReceived;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::types::aliases::{HeightOf, WriteAckEventOf};
use crate::components::default::relay::ReceivePacketRelayerComponent;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains, PacketOf};
use crate::relay::traits::packet_relayers::receive_packet::ReceivePacketRelayer;

pub struct SkipReceivedPacketRelayer<Relayer> {
    pub phantom: PhantomData<Relayer>,
}

#[cgp_provider(ReceivePacketRelayerComponent)]
impl<Relay, Relayer> ReceivePacketRelayer<Relay> for SkipReceivedPacketRelayer<Relayer>
where
    Relay: HasRelayChains + CanRaiseRelayChainErrors,
    Relayer: ReceivePacketRelayer<Relay>,
    Relay::SrcChain: HasPacketDstChannelId<Relay::DstChain>
        + HasPacketDstPortId<Relay::DstChain>
        + HasPacketSequence<Relay::DstChain>,
    Relay::DstChain: HasWriteAckEvent<Relay::SrcChain>,
    Relay::DstChain: CanQueryPacketIsReceived<Relay::SrcChain>,
{
    async fn relay_receive_packet(
        relay: &Relay,
        source_height: &HeightOf<Relay::SrcChain>,
        packet: &PacketOf<Relay>,
    ) -> Result<Option<WriteAckEventOf<Relay::DstChain, Relay::SrcChain>>, Relay::Error> {
        let is_packet_received = relay
            .dst_chain()
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
