use async_trait::async_trait;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_components::relay::traits::packet_lock::HasPacketLock;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

use crate::contexts::relay::CosmosRelay;
use crate::methods::packet_lock::try_acquire_packet_lock;
use crate::types::packet_lock::PacketLock;

#[async_trait]
impl<SrcChain, DstChain> HasPacketLock for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    type PacketLock<'a> = PacketLock;

    async fn try_acquire_packet_lock<'a>(&'a self, packet: &'a Packet) -> Option<PacketLock> {
        try_acquire_packet_lock(self, packet).await
    }
}
