use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::prelude::CanRaiseAsyncError;
use hermes_chain_type_components::traits::types::message_response::MessageResponseOf;
use hermes_relayer_components::chain::types::aliases::MessageOf;
use hermes_relayer_components::relay::traits::ibc_message_sender::{
    CanSendIbcMessages, IbcMessageSender,
};
use hermes_relayer_components::relay::traits::target::{HasTargetChainTypes, RelayTarget};
use hermes_runtime_components::traits::channel::CanUseChannels;
use hermes_runtime_components::traits::channel_once::{CanCreateChannelsOnce, CanUseChannelsOnce};

use crate::batch::traits::channel::HasMessageBatchSender;
use crate::batch::traits::types::CanUseMessageBatchChannel;
use crate::batch::types::sink::BatchWorkerSink;

pub struct SendMessagesToBatchWorker;

impl<Relay, Sink, Target> IbcMessageSender<Relay, Sink, Target> for SendMessagesToBatchWorker
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<Target>
        + CanSendIbcMessages<BatchWorkerSink, Target>
        + CanUseMessageBatchChannel<Target::Chain>
        + HasMessageBatchSender<Target::Chain>
        + CanRaiseAsyncError<ErrorOf<Relay::Runtime>>,
    Relay::Runtime: CanCreateChannelsOnce + CanUseChannelsOnce + CanUseChannels,
{
    async fn send_messages(
        relay: &Relay,
        _target: Target,
        messages: Vec<MessageOf<Relay::TargetChain>>,
    ) -> Result<Vec<MessageResponseOf<Relay::TargetChain>>, Relay::Error> {
        let (result_sender, result_receiver) = Relay::Runtime::new_channel_once();

        let message_sender = relay.get_batch_sender(PhantomData::<Target::Chain>);

        Relay::Runtime::send(message_sender, (messages, result_sender))
            .await
            .map_err(Relay::raise_error)?;

        let events = Relay::Runtime::receive_once(result_receiver)
            .await
            .map_err(Relay::raise_error)??;

        Ok(events)
    }
}
