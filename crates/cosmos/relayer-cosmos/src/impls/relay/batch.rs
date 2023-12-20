use async_trait::async_trait;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components_extra::batch::traits::channel::HasMessageBatchSender;
use ibc_relayer::chain::handle::ChainHandle;

use crate::contexts::relay::CosmosRelay;
use crate::types::batch::CosmosBatchSender;

#[async_trait]
impl<SrcChain, DstChain> HasMessageBatchSender<SourceTarget> for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    fn get_batch_sender(&self) -> &CosmosBatchSender {
        &self.src_chain_message_batch_sender
    }
}

#[async_trait]
impl<SrcChain, DstChain> HasMessageBatchSender<DestinationTarget>
    for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    fn get_batch_sender(&self) -> &CosmosBatchSender {
        &self.dst_chain_message_batch_sender
    }
}
