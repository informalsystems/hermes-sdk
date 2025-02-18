use cgp::prelude::*;
use hermes_chain_components::traits::packet::filter::{
    CanFilterIncomingPacket, CanFilterOutgoingPacket,
};

use crate::components::default::relay::RelayPacketFilterComponent;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains, PacketOf};
use crate::relay::traits::packet_filter::RelayPacketFilter;

pub struct FilterRelayPacketWithChains;

#[cgp_provider(RelayPacketFilterComponent)]
impl<Relay> RelayPacketFilter<Relay> for FilterRelayPacketWithChains
where
    Relay: HasRelayChains<
            SrcChain: CanFilterOutgoingPacket<Relay::DstChain>,
            DstChain: CanFilterIncomingPacket<Relay::SrcChain>,
        > + CanRaiseRelayChainErrors,
{
    async fn should_relay_packet(
        relay: &Relay,
        packet: &PacketOf<Relay>,
    ) -> Result<bool, Relay::Error> {
        let should_relay_src = relay
            .src_chain()
            .should_relay_outgoing_packet(packet)
            .await
            .map_err(Relay::raise_error)?;

        let should_relay = if should_relay_src {
            relay
                .dst_chain()
                .should_relay_incoming_packet(packet)
                .await
                .map_err(Relay::raise_error)?
        } else {
            false
        };

        Ok(should_relay)
    }
}
