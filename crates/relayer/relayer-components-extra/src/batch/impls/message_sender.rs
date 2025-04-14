use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::*;
use futures::channel::mpsc::{SendError, TrySendError};
use futures::channel::oneshot;
use futures::channel::oneshot::Canceled;
use hermes_chain_type_components::traits::types::message_response::MessageResponseOf;
use hermes_relayer_components::chain::types::aliases::MessageOf;
use hermes_relayer_components::relay::traits::ibc_message_sender::{
    CanSendIbcMessages, IbcMessageSender, IbcMessageSenderComponent,
};
use hermes_relayer_components::relay::traits::target::{HasTargetChainTypes, RelayTarget};

use crate::batch::traits::channel::HasMessageBatchSender;
use crate::batch::traits::types::CanUseMessageBatchChannel;
use crate::batch::types::sink::BatchWorkerSink;

#[cgp_new_provider(IbcMessageSenderComponent<Sink>)]
impl<Relay, Sink, Target> IbcMessageSender<Relay, Sink, Target> for SendMessagesToBatchWorker
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<Target>
        + CanSendIbcMessages<BatchWorkerSink, Target>
        + CanUseMessageBatchChannel<Target::Chain>
        + HasMessageBatchSender<Target::Chain>
        + CanRaiseAsyncError<SendError>
        + CanRaiseAsyncError<Canceled>,
{
    async fn send_messages(
        relay: &Relay,
        _target: Target,
        messages: Vec<MessageOf<Relay::TargetChain>>,
    ) -> Result<Vec<MessageResponseOf<Relay::TargetChain>>, Relay::Error> {
        let (result_sender, result_receiver) = oneshot::channel();

        let message_sender = relay.get_batch_sender(PhantomData::<Target::Chain>);

        message_sender
            .lock()
            .await
            .unbounded_send((messages, result_sender))
            .map_err(TrySendError::into_send_error)
            .map_err(Relay::raise_error)?;

        let events = result_receiver.await.map_err(Relay::raise_error)??;

        Ok(events)
    }
}
