use alloc::sync::Arc;
use std::collections::HashSet;

use futures::lock::Mutex;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::config::filter::PacketFilter;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ClientId, PortId};

use crate::contexts::chain::CosmosChain;
use crate::types::batch::CosmosBatchSender;

#[derive(Clone)]
pub struct CosmosRelay<SrcChain, DstChain> {
    pub runtime: HermesRuntime,
    pub src_chain: CosmosChain<SrcChain>,
    pub dst_chain: CosmosChain<DstChain>,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
    pub packet_filter: PacketFilter,
    pub packet_lock_mutex: Arc<Mutex<HashSet<(ChannelId, PortId, ChannelId, PortId, Sequence)>>>,
    pub src_chain_message_batch_sender: CosmosBatchSender,
    pub dst_chain_message_batch_sender: CosmosBatchSender,
}

impl<SrcChain, DstChain> CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    pub fn new(
        runtime: HermesRuntime,
        src_chain: CosmosChain<SrcChain>,
        dst_chain: CosmosChain<DstChain>,
        src_client_id: ClientId,
        dst_client_id: ClientId,
        packet_filter: PacketFilter,
        src_chain_message_batch_sender: CosmosBatchSender,
        dst_chain_message_batch_sender: CosmosBatchSender,
    ) -> Self {
        let relay = Self {
            runtime,
            src_chain,
            dst_chain,
            src_client_id,
            dst_client_id,
            packet_filter,
            src_chain_message_batch_sender,
            dst_chain_message_batch_sender,
            packet_lock_mutex: Arc::new(Mutex::new(HashSet::new())),
        };

        relay
    }
}
