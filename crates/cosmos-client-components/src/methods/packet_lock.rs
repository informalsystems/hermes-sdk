use alloc::sync::Arc;
use futures::channel::oneshot::channel;
use futures::lock::Mutex;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use std::collections::HashSet;
use tokio::runtime::Runtime;

use crate::types::packet_lock::PacketLock;

pub async fn try_acquire_packet_lock(
    runtime: &Runtime,
    packet_lock_mutex: &Arc<Mutex<HashSet<(ChannelId, PortId, ChannelId, PortId, Sequence)>>>,
    packet: &Packet,
) -> Option<PacketLock> {
    let packet_key = (
        packet.source_channel.clone(),
        packet.source_port.clone(),
        packet.destination_channel.clone(),
        packet.destination_port.clone(),
        packet.sequence,
    );

    let mut lock_table = packet_lock_mutex.lock().await;

    if lock_table.contains(&packet_key) {
        None
    } else {
        lock_table.insert(packet_key.clone());

        let (sender, receiver) = channel();

        let mutex = packet_lock_mutex.clone();

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
