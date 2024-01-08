use cgp_core::prelude::*;
use hermes_relayer_components::relay::traits::components::packet_filter::PacketFilter;

use ibc_relayer_types::core::ics04_channel::packet::Packet;

use crate::contexts::relay::CosmosRelay;
use crate::impls::relay::component::CosmosRelayComponents;
use crate::types::error::Error;

#[async_trait]
impl PacketFilter<CosmosRelay> for CosmosRelayComponents {
    async fn should_relay_packet(relay: &CosmosRelay, packet: &Packet) -> Result<bool, Error> {
        Ok(relay
            .packet_filter
            .channel_policy
            .is_allowed(&packet.source_port, &packet.source_channel))
    }
}
