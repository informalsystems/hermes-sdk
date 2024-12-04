use alloc::boxed::Box;
use alloc::vec;

use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::task::{CanRunConcurrentTasks, Task};

use super::ack::ClearAckPackets;
use super::receive_packet::ClearReceivePackets;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::{ChannelIdOf, PortIdOf};
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::packet_clearer::PacketClearer;

pub struct ClearAllPackets;

pub enum ClearOption {
    Receive,
    Ack,
}

pub struct RunPacketClearer<Relay>
where
    Relay: HasRelayChains,
{
    pub relay: Relay,
    pub src_channel_id: ChannelIdOf<Relay::SrcChain, Relay::DstChain>,
    pub src_port_id: PortIdOf<Relay::SrcChain, Relay::DstChain>,
    pub dst_channel_id: ChannelIdOf<Relay::DstChain, Relay::SrcChain>,
    pub dst_port_id: PortIdOf<Relay::DstChain, Relay::SrcChain>,
    pub clear_option: ClearOption,
}

impl<Relay> Task for RunPacketClearer<Relay>
where
    Relay: HasRelayChains,
    ClearReceivePackets: PacketClearer<Relay>,
    ClearAckPackets: PacketClearer<Relay>,
{
    async fn run(self) {
        let _ = match self.clear_option {
            ClearOption::Receive => {
                ClearReceivePackets::clear_packets(
                    &self.relay,
                    &self.src_channel_id,
                    &self.src_port_id,
                    &self.dst_channel_id,
                    &self.dst_port_id,
                )
                .await
            }
            ClearOption::Ack => {
                ClearAckPackets::clear_packets(
                    &self.relay,
                    &self.src_channel_id,
                    &self.src_port_id,
                    &self.dst_channel_id,
                    &self.dst_port_id,
                )
                .await
            }
        };
    }
}

impl<Relay, SrcChain, DstChain> PacketClearer<Relay> for ClearAllPackets
where
    Relay: Clone + HasRuntime + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>,
    SrcChain: HasIbcChainTypes<DstChain>,
    DstChain: HasIbcChainTypes<SrcChain>,
    Relay::Runtime: CanRunConcurrentTasks,
    SrcChain::ChannelId: Clone,
    SrcChain::PortId: Clone,
    DstChain::ChannelId: Clone,
    DstChain::PortId: Clone,
    RunPacketClearer<Relay>: Task,
{
    async fn clear_packets(
        relay: &Relay,
        src_channel_id: &SrcChain::ChannelId,
        src_port_id: &SrcChain::PortId,
        dst_channel_id: &DstChain::ChannelId,
        dst_port_id: &DstChain::PortId,
    ) -> Result<(), Relay::Error> {
        let receive_packet_task = Box::new(RunPacketClearer {
            relay: relay.clone(),
            src_channel_id: src_channel_id.clone(),
            src_port_id: src_port_id.clone(),
            dst_channel_id: dst_channel_id.clone(),
            dst_port_id: dst_port_id.clone(),
            clear_option: ClearOption::Receive,
        });

        let ack_packet_task = Box::new(RunPacketClearer {
            relay: relay.clone(),
            src_channel_id: src_channel_id.clone(),
            src_port_id: src_port_id.clone(),
            dst_channel_id: dst_channel_id.clone(),
            dst_port_id: dst_port_id.clone(),
            clear_option: ClearOption::Ack,
        });

        relay
            .runtime()
            .run_concurrent_tasks(vec![receive_packet_task, ack_packet_task])
            .await;

        Ok(())
    }
}
