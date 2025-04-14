use alloc::collections::VecDeque;
use alloc::format;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::mem;

use cgp::prelude::*;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LogLevel;
use hermes_relayer_components::chain::traits::types::message::{
    CanEstimateMessageSize, HasMessageType,
};
use hermes_relayer_components::multi::traits::chain_at::HasChainAt;
use hermes_relayer_components::relay::traits::ibc_message_sender::CanSendIbcMessages;
use hermes_relayer_components::relay::traits::target::RelayTarget;
use hermes_runtime_components::traits::channel::{CanUseChannels, HasChannelTypes};
use hermes_runtime_components::traits::channel_once::{CanUseChannelsOnce, HasChannelOnceTypes};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_runtime_components::traits::spawn::CanSpawnTask;
use hermes_runtime_components::traits::task::Task;
use hermes_runtime_components::traits::time::{HasTime, TimeOf};

use crate::batch::traits::types::{CanUseMessageBatchChannel, HasMessageBatchChannelTypes};
use crate::batch::types::config::BatchConfig;
use crate::batch::types::sink::BatchWorkerSink;

pub struct LogBatchWorker<'a, Target> {
    pub details: &'a str,
    pub log_level: LogLevel,
    pub phantom: PhantomData<Target>,
}

#[async_trait]
pub trait CanSpawnBatchMessageWorker<Target: RelayTarget>:
    HasMessageBatchChannelTypes<Target::Chain> + HasAsyncErrorType
{
    fn spawn_batch_message_worker(
        &self,
        _target: Target,
        config: BatchConfig,
        receiver: Self::MessageBatchReceiver,
    );
}

impl<Relay, Target> CanSpawnBatchMessageWorker<Target> for Relay
where
    Target: RelayTarget,
    Relay: Clone + HasRuntime + CanRunLoop<Target> + CanUseMessageBatchChannel<Target::Chain>,
    Relay::Runtime: CanSpawnTask,
{
    fn spawn_batch_message_worker(
        &self,
        _target: Target,
        config: BatchConfig,
        receiver: Relay::MessageBatchReceiver,
    ) {
        let task = BatchMessageTask {
            relay: self.clone(),
            config,
            receiver,
            phantom: PhantomData,
        };

        self.runtime().spawn_task(task);
    }
}

pub struct BatchMessageTask<Relay, Target>
where
    Target: RelayTarget,
    Relay: HasMessageBatchChannelTypes<Target::Chain>,
{
    pub relay: Relay,
    pub config: BatchConfig,
    pub receiver: Relay::MessageBatchReceiver,
    pub phantom: PhantomData<Target>,
}

impl<Relay, Target> Task for BatchMessageTask<Relay, Target>
where
    Target: RelayTarget,
    Relay: HasMessageBatchChannelTypes<Target::Chain> + CanRunLoop<Target>,
{
    async fn run(self) {
        self.relay.run_loop(&self.config, self.receiver).await;
    }
}

#[async_trait]
pub trait CanRunLoop<Target: RelayTarget>: HasMessageBatchChannelTypes<Target::Chain> {
    async fn run_loop(&self, config: &BatchConfig, receiver: Self::MessageBatchReceiver);
}

impl<Relay, Target> CanRunLoop<Target> for Relay
where
    Target: RelayTarget,
    Relay: HasChainAt<Target::Chain>
        + CanUseMessageBatchChannel<Target::Chain>
        + CanProcessMessageBatches<Target>
        + for<'a> CanLog<LogBatchWorker<'a, Target>>,
    Relay::Runtime: HasTime + CanSleep + CanUseChannels + HasChannelOnceTypes,
{
    async fn run_loop(&self, config: &BatchConfig, mut receiver: Relay::MessageBatchReceiver) {
        let runtime = self.runtime();

        let mut pending_batches: VecDeque<Relay::BatchSubmission> = VecDeque::new();

        let mut last_sent_time = runtime.now();

        loop {
            let payload = Relay::Runtime::try_receive(&mut receiver);

            match payload {
                Ok(m_batch) => {
                    if let Some(batch) = m_batch {
                        let batch_size = batch.0.len();

                        self.log(
                            "received message batch",
                            &LogBatchWorker {
                                details: &format!("batch_size = {batch_size}"),
                                log_level: LogLevel::Trace,
                                phantom: PhantomData,
                            },
                        )
                        .await;

                        pending_batches.push_back(batch);
                    }

                    let current_batch_size = pending_batches.len();
                    let now = runtime.now();

                    self.process_message_batches(
                        config,
                        &mut pending_batches,
                        now,
                        &mut last_sent_time,
                    )
                    .await;

                    if pending_batches.len() == current_batch_size {
                        runtime.sleep(config.sleep_time).await;
                    }
                }
                Err(e) => {
                    self.log(
                        "error in try_receive, terminating worker",
                        &LogBatchWorker {
                            details: &format!("error = {:?}", e),
                            log_level: LogLevel::Error,
                            phantom: PhantomData,
                        },
                    )
                    .await;

                    return;
                }
            }
        }
    }
}

#[async_trait]
pub trait CanProcessMessageBatches<Target: RelayTarget>:
    HasMessageBatchChannelTypes<Target::Chain> + HasRuntime<Runtime: HasTime>
{
    async fn process_message_batches(
        &self,
        config: &BatchConfig,
        pending_batches: &mut VecDeque<Self::BatchSubmission>,
        now: TimeOf<Self::Runtime>,
        last_sent_time: &mut TimeOf<Self::Runtime>,
    );
}

impl<Relay, Target> CanProcessMessageBatches<Target> for Relay
where
    Target: RelayTarget,
    Relay: Clone
        + HasRuntime
        + CanUseMessageBatchChannel<Target::Chain>
        + CanPartitionMessageBatches<Target>
        + for<'a> CanLog<LogBatchWorker<'a, Target>>,
    Relay::Runtime:
        HasTime + CanSpawnTask + HasChannelTypes + HasChannelOnceTypes + HasAsyncErrorType,
    SendReadyBatchTask<Relay, Target>: Task,
{
    async fn process_message_batches(
        &self,
        config: &BatchConfig,
        pending_batches: &mut VecDeque<Relay::BatchSubmission>,
        now: TimeOf<Relay::Runtime>,
        last_sent_time: &mut TimeOf<Relay::Runtime>,
    ) {
        let ready_batches = Relay::partition_message_batches(config, pending_batches);

        if ready_batches.is_empty() {
            // If there is nothing to send, return the remaining batches which should also be empty
        } else if pending_batches.is_empty()
            && Relay::Runtime::duration_since(&now, last_sent_time) < config.max_delay
        {
            // If the current batch is not full and there is still some time until max delay,
            // return everything and wait until the next batch is full
            *pending_batches = ready_batches;
        } else {
            let batch_size = ready_batches.len();

            self.log(
                "sending ready batches",
                &LogBatchWorker {
                    details: &format!("batch_size = {batch_size}"),
                    log_level: LogLevel::Trace,
                    phantom: PhantomData,
                },
            )
            .await;

            let task = SendReadyBatchTask {
                relay: self.clone(),
                ready_batches,
            };

            self.runtime().spawn_task(task);

            *last_sent_time = now;
        }
    }
}

pub struct SendReadyBatchTask<Relay, Target>
where
    Target: RelayTarget,
    Relay: HasMessageBatchChannelTypes<Target::Chain>,
{
    pub relay: Relay,
    pub ready_batches: VecDeque<Relay::BatchSubmission>,
}

impl<Relay, Target> Task for SendReadyBatchTask<Relay, Target>
where
    Target: RelayTarget,
    Relay: CanSendReadyBatches<Target>,
{
    async fn run(self) {
        self.relay.send_ready_batches(self.ready_batches).await
    }
}

pub trait CanPartitionMessageBatches<Target: RelayTarget>:
    HasMessageBatchChannelTypes<Target::Chain>
{
    fn partition_message_batches(
        config: &BatchConfig,
        pending_batches: &mut VecDeque<Self::BatchSubmission>,
    ) -> VecDeque<Self::BatchSubmission>;
}

impl<Relay, Target> CanPartitionMessageBatches<Target> for Relay
where
    Target: RelayTarget,
    Relay: CanUseMessageBatchChannel<Target::Chain>,
    Relay::Chain: CanEstimateBatchSize,
{
    fn partition_message_batches(
        config: &BatchConfig,
        pending_batches: &mut VecDeque<Relay::BatchSubmission>,
    ) -> VecDeque<Relay::BatchSubmission> {
        let batches = mem::take(pending_batches);

        let mut total_message_count: usize = 0;
        let mut total_batch_size: usize = 0;

        let (mut ready_batches, mut remaining_batches): (VecDeque<_>, _) = batches
            .into_iter()
            .partition(|(current_messages, _sender)| {
                if total_message_count > config.max_message_count
                    || total_batch_size > config.max_tx_size
                {
                    false
                } else {
                    let current_message_count = current_messages.len();
                    let current_batch_size = Relay::Chain::estimate_batch_size(current_messages);

                    if total_message_count + current_message_count > config.max_message_count
                        || total_batch_size + current_batch_size > config.max_tx_size
                    {
                        false
                    } else {
                        total_message_count += current_message_count;
                        total_batch_size += current_batch_size;

                        true
                    }
                }
            });

        // If for some reason ready batch is empty but remaining batches is not,
        // it means there are single batch that are too big to fit in.
        // In that case put the first remaining batch as ready.
        if ready_batches.is_empty() && !remaining_batches.is_empty() {
            if let Some(batch) = remaining_batches.pop_front() {
                ready_batches.push_back(batch);
            }
        }

        *pending_batches = remaining_batches;

        ready_batches
    }
}

#[async_trait]
pub trait CanSendReadyBatches<Target: RelayTarget>:
    HasMessageBatchChannelTypes<Target::Chain>
{
    async fn send_ready_batches(&self, ready_batches: VecDeque<Self::BatchSubmission>);
}

impl<Relay, Target> CanSendReadyBatches<Target> for Relay
where
    Target: RelayTarget,
    Relay: CanUseMessageBatchChannel<Target::Chain>
        + CanSendIbcMessages<BatchWorkerSink, Target>
        + for<'a> CanLog<LogBatchWorker<'a, Target>>,
    Relay::Runtime: CanUseChannelsOnce + CanUseChannels,
    Relay::Error: Clone,
{
    async fn send_ready_batches(&self, ready_batches: VecDeque<Relay::BatchSubmission>) {
        let (messages, senders): (Vec<_>, Vec<_>) = ready_batches
            .into_iter()
            .map(|(messages, result_sender)| {
                let message_count = messages.len();
                (messages, (message_count, result_sender))
            })
            .unzip();

        let in_messages = messages.into_iter().flatten().collect::<Vec<_>>();

        let message_count = in_messages.len();

        self.log(
            "sending batched messages to inner sender",
            &LogBatchWorker {
                details: &format!("message_count = {message_count}"),
                log_level: LogLevel::Trace,
                phantom: PhantomData,
            },
        )
        .await;

        let send_result = self.send_messages(Target::default(), in_messages).await;

        match send_result {
            Err(e) => {
                self.log(
                    "inner sender returned error result, sending error back to caller",
                    &LogBatchWorker {
                        details: &format!("error = {:?}", e),
                        log_level: LogLevel::Trace,
                        phantom: PhantomData,
                    },
                )
                .await;

                for (_, sender) in senders.into_iter() {
                    let _ = Relay::Runtime::send_once(sender, Err(e.clone()));
                }
            }
            Ok(all_events) => {
                let events_count = all_events.len();
                let mut all_events = all_events.into_iter();

                self.log(
                    "inner sender returned result events, sending events back to caller",
                    &LogBatchWorker {
                        details: &format!("events_count = {events_count}"),
                        log_level: LogLevel::Trace,
                        phantom: PhantomData,
                    },
                )
                .await;

                for (message_count, sender) in senders.into_iter() {
                    let events = take(&mut all_events, message_count);
                    let _ = Relay::Runtime::send_once(sender, Ok(events));
                }
            }
        }
    }
}

trait CanEstimateBatchSize: HasMessageType {
    fn estimate_batch_size(messages: &[Self::Message]) -> usize;
}

impl<Chain> CanEstimateBatchSize for Chain
where
    Chain: CanEstimateMessageSize,
{
    fn estimate_batch_size(messages: &[Self::Message]) -> usize {
        messages
            .iter()
            .map(|message| {
                // return 0 on encoding error, as we don't want
                // the batching operation to error out.
                Chain::estimate_message_size(message).unwrap_or(0)
            })
            .sum()
    }
}

fn take<T, I: Iterator<Item = T>>(it: &mut I, count: usize) -> Vec<T> {
    let mut res = Vec::new();
    for _ in 0..count {
        match it.next() {
            Some(x) => {
                res.push(x);
            }
            None => {
                return res;
            }
        }
    }
    res
}
