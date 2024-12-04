use alloc::vec::Vec;

use cgp::prelude::CanRaiseError;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::relay::traits::ibc_message_sender::{
    CanSendIbcMessages, IbcMessageSender,
};
use hermes_relayer_components::relay::traits::target::{HasTargetChainTypes, RelayTarget};
use hermes_runtime_components::traits::channel::CanUseChannels;
use hermes_runtime_components::traits::channel_once::{CanCreateChannelsOnce, CanUseChannelsOnce};
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::batch::traits::channel::HasMessageBatchSender;
use crate::batch::types::sink::BatchWorkerSink;

pub struct SendMessagesToBatchWorker;

impl<Relay, Sink, Target, TargetChain, Runtime> IbcMessageSender<Relay, Sink, Target>
    for SendMessagesToBatchWorker
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<Target, TargetChain = TargetChain>
        + CanSendIbcMessages<BatchWorkerSink, Target>
        + CanRaiseError<Runtime::Error>,
    TargetChain: HasIbcChainTypes<Relay::CounterpartyChain>,
    TargetChain: HasRuntime<Runtime = Runtime>,
    Runtime: CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
    Relay: HasMessageBatchSender<Target>,
{
    async fn send_messages(
        context: &Relay,
        _target: Target,
        messages: Vec<TargetChain::Message>,
    ) -> Result<Vec<TargetChain::MessageResponse>, Relay::Error> {
        let (result_sender, result_receiver) = Runtime::new_channel_once();

        let message_sender = context.get_batch_sender();

        Runtime::send(message_sender, (messages, result_sender))
            .await
            .map_err(Relay::raise_error)?;

        let events = Runtime::receive_once(result_receiver)
            .await
            .map_err(Relay::raise_error)??;

        Ok(events)
    }
}
