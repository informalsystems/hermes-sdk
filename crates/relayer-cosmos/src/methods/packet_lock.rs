use futures::channel::oneshot::channel;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

use crate::contexts::relay::CosmosRelay;
use crate::types::packet_lock::PacketLock;

pub async fn try_acquire_packet_lock<SrcChain, DstChain>(
    relay: &CosmosRelay<SrcChain, DstChain>,
    packet: &Packet,
) -> Option<PacketLock> {
    let packet_key = (
        packet.source_channel.clone(),
        packet.source_port.clone(),
        packet.destination_channel.clone(),
        packet.destination_port.clone(),
        packet.sequence,
    );

    let mutex = &relay.packet_lock_mutex;

    let mut lock_table = mutex.lock().await;

    if lock_table.contains(&packet_key) {
        None
    } else {
        lock_table.insert(packet_key.clone());

        let runtime = &relay.runtime.runtime;

        let (sender, receiver) = channel();

        let mutex = mutex.clone();

        runtime.spawn(async move {
            let _ = receiver.await;
            let mut lock_table = mutex.lock().await;
            lock_table.remove(&packet_key);
        });

        Some(PacketLock {
            release_sender: Some(sender),
        })
    }
}
