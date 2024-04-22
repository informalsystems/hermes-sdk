use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components_extra::batch::traits::channel::MessageBatchSenderGetter;

use crate::contexts::relay::CosmosRelay;
use crate::impls::relay::component::CosmosRelayComponents;
use crate::types::batch::CosmosBatchSender;

impl MessageBatchSenderGetter<CosmosRelay, SourceTarget> for CosmosRelayComponents {
    fn get_batch_sender(relay: &CosmosRelay) -> &CosmosBatchSender {
        &relay.src_chain_message_batch_sender
    }
}

impl MessageBatchSenderGetter<CosmosRelay, DestinationTarget> for CosmosRelayComponents {
    fn get_batch_sender(relay: &CosmosRelay) -> &CosmosBatchSender {
        &relay.dst_chain_message_batch_sender
    }
}
