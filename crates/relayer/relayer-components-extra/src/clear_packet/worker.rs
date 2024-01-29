use core::time::Duration;

use cgp_core::async_trait;
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, PortIdOf};
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::clear_interval::HasClearInterval;
use hermes_relayer_components::relay::traits::components::packet_clearer::CanClearPackets;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_relayer_components::runtime::traits::sleep::CanSleep;
use hermes_relayer_components::runtime::traits::task::Task;

use crate::runtime::traits::spawn::CanSpawnTask;

#[async_trait]
pub trait CanSpawnClearPacketWorker: HasRelayChains {
    fn spawn_packet_clear_worker(
        &self,
        src_channel_id: ChannelIdOf<Self::SrcChain, Self::DstChain>,
        src_port_id: PortIdOf<Self::SrcChain, Self::DstChain>,
        dst_channel_id: ChannelIdOf<Self::DstChain, Self::SrcChain>,
        dst_port_id: PortIdOf<Self::DstChain, Self::SrcChain>,
    );
}

pub struct ClearPacketTask<Relay>
where
    Relay: HasRelayChains,
{
    pub relay: Relay,
    pub src_channel_id: ChannelIdOf<Relay::SrcChain, Relay::DstChain>,
    pub src_port_id: PortIdOf<Relay::SrcChain, Relay::DstChain>,
    pub dst_channel_id: ChannelIdOf<Relay::DstChain, Relay::SrcChain>,
    pub dst_port_id: PortIdOf<Relay::DstChain, Relay::SrcChain>,
}

#[async_trait]
impl<Relay> Task for ClearPacketTask<Relay>
where
    Relay: CanRunLoop,
{
    async fn run(self) {
        self.relay
            .run_loop(
                &self.src_channel_id,
                &self.src_port_id,
                &self.dst_channel_id,
                &self.dst_port_id,
            )
            .await;
    }
}

impl<Relay> CanSpawnClearPacketWorker for Relay
where
    Relay: Clone + CanRunLoop + HasRuntime,
    Relay::Runtime: CanSpawnTask,
{
    fn spawn_packet_clear_worker(
        &self,
        src_channel_id: ChannelIdOf<Relay::SrcChain, Relay::DstChain>,
        src_port_id: PortIdOf<Relay::SrcChain, Relay::DstChain>,
        dst_channel_id: ChannelIdOf<Relay::DstChain, Relay::SrcChain>,
        dst_port_id: PortIdOf<Relay::DstChain, Relay::SrcChain>,
    ) {
        let task = ClearPacketTask {
            relay: self.clone(),
            src_channel_id,
            src_port_id,
            dst_channel_id,
            dst_port_id,
        };

        self.runtime().spawn_task(task);
    }
}

#[async_trait]
trait CanRunLoop: HasRelayChains {
    async fn run_loop(
        &self,
        src_channel_id: &ChannelIdOf<Self::SrcChain, Self::DstChain>,
        src_port_id: &PortIdOf<Self::SrcChain, Self::DstChain>,
        dst_channel_id: &ChannelIdOf<Self::DstChain, Self::SrcChain>,
        dst_port_id: &PortIdOf<Self::DstChain, Self::SrcChain>,
    );
}

#[async_trait]
impl<Relay> CanRunLoop for Relay
where
    Relay: HasRuntime + CanClearPackets + HasClearInterval,
    Relay::Runtime: CanSleep,
{
    async fn run_loop(
        &self,
        src_channel_id: &ChannelIdOf<Relay::SrcChain, Relay::DstChain>,
        src_port_id: &PortIdOf<Relay::SrcChain, Relay::DstChain>,
        dst_channel_id: &ChannelIdOf<Relay::DstChain, Relay::SrcChain>,
        dst_port_id: &PortIdOf<Relay::DstChain, Relay::SrcChain>,
    ) {
        let runtime = self.runtime();
        let clear_interval = self.clear_interval().into();

        loop {
            let _ = self
                .clear_packets(src_channel_id, src_port_id, dst_channel_id, dst_port_id)
                .await;

            runtime.sleep(Duration::from_secs(clear_interval)).await;
        }
    }
}
