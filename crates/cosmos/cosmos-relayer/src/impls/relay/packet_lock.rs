use cgp_core::prelude::*;
use hermes_cosmos_chain_components::methods::packet_lock::try_acquire_packet_lock;
use hermes_cosmos_chain_components::types::packet_lock::PacketLock;
use hermes_relayer_components::relay::traits::packet_lock::HasPacketLock;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

use crate::contexts::relay::CosmosRelay;

#[async_trait]
impl HasPacketLock for CosmosRelay {
    type PacketLock<'a> = PacketLock;

    async fn try_acquire_packet_lock<'a>(&'a self, packet: &'a Packet) -> Option<PacketLock> {
        try_acquire_packet_lock(&self.runtime.runtime, &self.packet_lock_mutex, packet).await
    }
}
