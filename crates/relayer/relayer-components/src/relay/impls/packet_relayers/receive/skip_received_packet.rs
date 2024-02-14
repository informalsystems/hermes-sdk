use core::marker::PhantomData;

use cgp_core::async_trait;

use crate::chain::traits::queries::packet_is_received::CanQueryPacketIsReceived;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::types::aliases::{HeightOf, WriteAckEventOf};
use crate::relay::traits::chains::CanRaiseRelayChainErrors;
use crate::relay::traits::packet::HasRelayPacketFields;
use crate::relay::traits::packet_relayers::receive_packet::ReceivePacketRelayer;

pub struct SkipReceivedPacketRelayer<Relayer> {
    pub phantom: PhantomData<Relayer>,
}

#[async_trait]
impl<Relay, Relayer> ReceivePacketRelayer<Relay> for SkipReceivedPacketRelayer<Relayer>
where
    Relay: HasRelayPacketFields + CanRaiseRelayChainErrors,
    Relayer: ReceivePacketRelayer<Relay>,
    Relay::DstChain: HasWriteAckEvent<Relay::SrcChain>,
    Relay::DstChain: CanQueryPacketIsReceived<Relay::SrcChain>,
{
    async fn relay_receive_packet(
        relay: &Relay,
        source_height: &HeightOf<Relay::SrcChain>,
        packet: &Relay::Packet,
    ) -> Result<Option<WriteAckEventOf<Relay::DstChain, Relay::SrcChain>>, Relay::Error> {
        let is_packet_received = relay
            .dst_chain()
            .query_packet_is_received(
                Relay::packet_dst_port(packet),
                Relay::packet_dst_channel_id(packet),
                Relay::packet_sequence(packet),
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
