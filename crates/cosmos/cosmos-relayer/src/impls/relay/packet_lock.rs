use hermes_cosmos_chain_components::methods::packet_lock::try_acquire_packet_lock;
use hermes_cosmos_chain_components::types::packet_lock::PacketLock;
use hermes_relayer_components::relay::traits::packet_lock::ProvidePacketLock;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

use crate::contexts::relay::CosmosRelay;
use crate::impls::relay::component::CosmosRelayComponents;

impl ProvidePacketLock<CosmosRelay> for CosmosRelayComponents {
    type PacketLock<'a> = PacketLock;

    async fn try_acquire_packet_lock<'a>(
        relay: &'a CosmosRelay,
        packet: &'a Packet,
    ) -> Option<PacketLock> {
        try_acquire_packet_lock(&relay.runtime.runtime, &relay.packet_lock_mutex, packet).await
    }
}
