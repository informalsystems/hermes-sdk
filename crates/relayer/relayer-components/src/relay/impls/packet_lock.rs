use alloc::collections::BTreeSet;
use alloc::sync::Arc;
use core::marker::PhantomData;

use cgp::core::field::UseField;
use cgp::core::macros::blanket_trait;
use futures::channel::oneshot::{channel, Receiver, Sender};
use futures::lock::Mutex;
use hermes_chain_components::traits::{
    HasChannelIdType, HasPacketDstChannelId, HasPacketDstPortId, HasPacketSequence,
    HasPacketSrcChannelId, HasPacketSrcPortId, HasPortIdType, HasSequenceType,
};
use hermes_prelude::*;
use hermes_runtime_components::traits::{CanSpawnTask, HasRuntime, HasRuntimeType, Task};

use crate::chain::types::aliases::{ChannelIdOf, PortIdOf, SequenceOf};
use crate::relay::traits::{
    HasRelayChainTypes, HasRelayChains, PacketLockComponent, PacketOf, ProvidePacketLock,
};

pub struct ProvidePacketLockWithMutex;

pub struct PacketLock {
    pub release_sender: Option<Sender<()>>,
}

pub trait HasPacketMutexType: Async {
    type PacketKey: Async + Ord + Clone;

    type PacketMutex: Async;
}

pub type PacketMutexOf<Relay> = <Relay as HasPacketMutexType>::PacketMutex;

impl<Relay, Runtime, SrcChain, DstChain> HasPacketMutexType for Relay
where
    Relay: HasRuntimeType<Runtime = Runtime>
        + HasRelayChainTypes<SrcChain = SrcChain, DstChain = DstChain>,
    SrcChain: HasChannelIdType<DstChain, ChannelId: Ord + Clone>
        + HasPortIdType<DstChain, PortId: Ord + Clone>
        + HasSequenceType<DstChain, Sequence: Ord + Clone>
        + HasAsyncErrorType,
    DstChain: HasChannelIdType<SrcChain, ChannelId: Ord + Clone>
        + HasPortIdType<SrcChain, PortId: Ord + Clone>
        + HasAsyncErrorType,
{
    type PacketKey = (
        SrcChain::ChannelId,
        SrcChain::PortId,
        DstChain::ChannelId,
        DstChain::PortId,
        SrcChain::Sequence,
    );

    type PacketMutex = Arc<Mutex<BTreeSet<Self::PacketKey>>>;
}

#[blanket_trait]
pub trait CanUsePacketMutex:
    HasRelayChainTypes<
        SrcChain: HasChannelIdType<Self::DstChain, ChannelId: Ord + Clone>
                      + HasPortIdType<Self::DstChain, PortId: Ord + Clone>
                      + HasSequenceType<Self::DstChain, Sequence: Ord + Clone>,
        DstChain: HasChannelIdType<Self::SrcChain, ChannelId: Ord + Clone>
                      + HasPortIdType<Self::SrcChain, PortId: Ord + Clone>,
    > + HasPacketMutexType<
        PacketKey = (
            ChannelIdOf<Self::SrcChain, Self::DstChain>,
            PortIdOf<Self::SrcChain, Self::DstChain>,
            ChannelIdOf<Self::DstChain, Self::SrcChain>,
            PortIdOf<Self::DstChain, Self::SrcChain>,
            SequenceOf<Self::SrcChain, Self::DstChain>,
        ),
        PacketMutex = Arc<
            Mutex<
                BTreeSet<(
                    ChannelIdOf<Self::SrcChain, Self::DstChain>,
                    PortIdOf<Self::SrcChain, Self::DstChain>,
                    ChannelIdOf<Self::DstChain, Self::SrcChain>,
                    PortIdOf<Self::DstChain, Self::SrcChain>,
                    SequenceOf<Self::SrcChain, Self::DstChain>,
                )>,
            >,
        >,
    >
{
}

#[cgp_component {
  provider: PacketMutexGetter,
  context: Relay,
}]
pub trait HasPacketMutex: HasPacketMutexType {
    fn packet_mutex(&self) -> &Self::PacketMutex;
}

pub struct ReleasePacketLockTask<Relay>
where
    Relay: HasPacketMutexType,
{
    pub release_receiver: Receiver<()>,
    pub packet_mutex: Relay::PacketMutex,
    pub packet_key: Relay::PacketKey,
}

impl<Relay> Task for ReleasePacketLockTask<Relay>
where
    Relay: CanUsePacketMutex,
{
    async fn run(self) {
        let _ = self.release_receiver.await;
        let mut lock_table = self.packet_mutex.lock().await;
        lock_table.remove(&self.packet_key);
    }
}

#[cgp_provider(PacketLockComponent)]
impl<Relay> ProvidePacketLock<Relay> for ProvidePacketLockWithMutex
where
    Relay: HasRuntime + CanUsePacketMutex + HasPacketMutex + HasRelayChains,
    Relay::Runtime: CanSpawnTask,
    Relay::SrcChain: HasPacketSrcChannelId<Relay::DstChain>
        + HasPacketSrcPortId<Relay::DstChain>
        + HasPacketDstChannelId<Relay::DstChain>
        + HasPacketDstPortId<Relay::DstChain>
        + HasPacketSequence<Relay::DstChain>,
{
    type PacketLock<'a> = PacketLock;

    async fn try_acquire_packet_lock<'a>(
        relay: &'a Relay,
        packet: &'a PacketOf<Relay>,
    ) -> Option<PacketLock> {
        let packet_key = (
            Relay::SrcChain::packet_src_channel_id(packet),
            Relay::SrcChain::packet_src_port_id(packet),
            Relay::SrcChain::packet_dst_channel_id(packet),
            Relay::SrcChain::packet_dst_port_id(packet),
            Relay::SrcChain::packet_sequence(packet),
        );

        let packet_mutex = relay.packet_mutex();

        let mut lock_table = packet_mutex.lock().await;

        if lock_table.contains(&packet_key) {
            None
        } else {
            lock_table.insert(packet_key.clone());

            let (sender, receiver) = channel();

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

#[cgp_provider(PacketMutexGetterComponent)]
impl<Relay, FieldTag> PacketMutexGetter<Relay> for UseField<FieldTag>
where
    Relay: HasPacketMutexType + HasField<FieldTag, Value = Relay::PacketMutex>,
{
    fn packet_mutex(relay: &Relay) -> &Relay::PacketMutex {
        relay.get_field(PhantomData)
    }
}

impl Drop for PacketLock {
    fn drop(&mut self) {
        if let Some(sender) = self.release_sender.take() {
            let _ = sender.send(());
        }
    }
}
