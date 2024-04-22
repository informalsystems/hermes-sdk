use alloc::collections::BTreeSet;
use alloc::sync::Arc;
use cgp_core::Async;
use hermes_runtime_components::traits::channel_once::CanUseChannelsOnce;
use hermes_runtime_components::traits::channel_once::{
    CanCreateChannelsOnce, HasChannelOnceTypes, ReceiverOnce, SenderOnceOf,
};
use hermes_runtime_components::traits::mutex::{HasMutex, MutexOf};
use hermes_runtime_components::traits::runtime::{HasRuntime, RuntimeOf};
use hermes_runtime_components::traits::spawn::CanSpawnTask;
use hermes_runtime_components::traits::task::Task;

use crate::chain::traits::packet::fields::CanReadPacketFields;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::{ChannelIdOf, PortIdOf, SequenceOf};
use crate::relay::traits::chains::{DstChainOf, HasRelayChains, SrcChainOf};
use crate::relay::traits::packet_lock::ProvidePacketLock;
pub struct ProvidePacketLockWithMutex;

pub struct PacketLock<Relay>
where
    Relay: HasRuntime,
    Relay::Runtime: CanUseChannelsOnce,
{
    pub release_sender: Option<SenderOnceOf<Relay::Runtime, ()>>,
}

pub type PacketKey<Relay> = (
    ChannelIdOf<SrcChainOf<Relay>, DstChainOf<Relay>>,
    PortIdOf<SrcChainOf<Relay>, DstChainOf<Relay>>,
    ChannelIdOf<DstChainOf<Relay>, SrcChainOf<Relay>>,
    PortIdOf<DstChainOf<Relay>, SrcChainOf<Relay>>,
    SequenceOf<SrcChainOf<Relay>, DstChainOf<Relay>>,
);

pub type PacketMutex<Relay> = Arc<MutexOf<RuntimeOf<Relay>, BTreeSet<PacketKey<Relay>>>>;

pub trait HasPacketMutex: HasRuntime + HasRelayChains
where
    Self::Runtime: HasMutex,
{
    fn packet_mutex(&self) -> &PacketMutex<Self>;
}
pub struct ReleasePacketLockTask<Relay>
where
    Relay: HasRuntime + HasRelayChains,
    Relay::Runtime: HasMutex + HasChannelOnceTypes,
{
    pub release_receiver: ReceiverOnce<Relay::Runtime, ()>,
    pub packet_mutex: PacketMutex<Relay>,
    pub packet_key: PacketKey<Relay>,
}

impl<Relay, Runtime> Task for ReleasePacketLockTask<Relay>
where
    Relay: HasRuntime<Runtime = Runtime> + HasRelayChains,
    Runtime: HasMutex + CanUseChannelsOnce,
    PacketKey<Relay>: Ord,
{
    async fn run(self) {
        let _ = Runtime::receive_once(self.release_receiver).await;
        let mut lock_table = Runtime::acquire_mutex(&self.packet_mutex).await;
        lock_table.remove(&self.packet_key);
    }
}

impl<Relay, SrcChain, DstChain, Packet, Runtime> ProvidePacketLock<Relay>
    for ProvidePacketLockWithMutex
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain, Packet = Packet>
        + HasRuntime<Runtime = Runtime>
        + HasPacketMutex,
    Runtime: HasMutex + CanUseChannelsOnce + CanCreateChannelsOnce + CanSpawnTask,
    SrcChain: CanReadPacketFields<DstChain, OutgoingPacket = Packet>,
    DstChain: HasIbcChainTypes<SrcChain>,
    SrcChain::ChannelId: Clone + Ord,
    SrcChain::PortId: Clone + Ord,
    SrcChain::Sequence: Clone + Ord,
    DstChain::ChannelId: Clone + Ord,
    DstChain::PortId: Clone + Ord,
    Packet: Async,
{
    type PacketLock<'a> = PacketLock<Relay>;

    async fn try_acquire_packet_lock<'a>(
        relay: &'a Relay,
        packet: &'a Relay::Packet,
    ) -> Option<PacketLock<Relay>> {
        let packet_key: PacketKey<Relay> = (
            SrcChain::outgoing_packet_src_channel_id(packet).clone(),
            SrcChain::outgoing_packet_src_port(packet).clone(),
            SrcChain::outgoing_packet_dst_channel_id(packet).clone(),
            SrcChain::outgoing_packet_dst_port(packet).clone(),
            SrcChain::outgoing_packet_sequence(packet).clone(),
        );

        let packet_mutex = relay.packet_mutex();

        let mut lock_table = Runtime::acquire_mutex(packet_mutex).await;

        if lock_table.contains(&packet_key) {
            None
        } else {
            lock_table.insert(packet_key.clone());

            let (sender, receiver) = Runtime::new_channel_once();

            let release_task: ReleasePacketLockTask<Relay> = ReleasePacketLockTask {
                release_receiver: receiver,
                packet_mutex: packet_mutex.clone(),
                packet_key,
            };

            relay.runtime().spawn_task(release_task);

            Some(PacketLock {
                release_sender: Some(sender),
            })
        }
    }
}

impl<Relay> Drop for PacketLock<Relay>
where
    Relay: HasRuntime,
    Relay::Runtime: CanUseChannelsOnce,
{
    fn drop(&mut self) {
        if let Some(sender) = self.release_sender.take() {
            let _ = Relay::Runtime::send_once(sender, ());
        }
    }
}
