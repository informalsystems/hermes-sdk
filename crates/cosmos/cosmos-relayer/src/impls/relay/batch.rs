use cgp_core::prelude::*;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components_extra::batch::traits::channel::HasMessageBatchSender;

use crate::contexts::relay::CosmosRelay;
use crate::types::batch::CosmosBatchSender;

#[async_trait]
impl HasMessageBatchSender<SourceTarget> for CosmosRelay {
    fn get_batch_sender(&self) -> &CosmosBatchSender {
        &self.src_chain_message_batch_sender
    }
}

#[async_trait]
impl HasMessageBatchSender<DestinationTarget> for CosmosRelay {
    fn get_batch_sender(&self) -> &CosmosBatchSender {
        &self.dst_chain_message_batch_sender
    }
}
