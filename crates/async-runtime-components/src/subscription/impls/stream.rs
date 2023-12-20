use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ops::DerefMut;

use alloc::boxed::Box;
use cgp_core::prelude::*;
use futures_core::stream::Stream;
use futures_util::stream::StreamExt;
use ibc_relayer_components::runtime::traits::mutex::HasMutex;
use ibc_relayer_components::runtime::traits::task::Task;
use ibc_relayer_components_extra::runtime::traits::channel::{
    CanCreateChannels, CanStreamReceiver, CanUseChannels, HasChannelTypes,
};
use ibc_relayer_components_extra::runtime::traits::spawn::CanSpawnTask;

use crate::stream::traits::boxed::HasBoxedStreamType;
use crate::subscription::impls::multiplex::MultiplexingSubscription;
use crate::subscription::traits::subscription::Subscription;

/**
   Allows multiplexing of a single [`Stream`] into a subscription.
   This is an auto trait implemented by all runtime contexts that implement
   [`HasSpawner`], [`HasMutex`], [`CanCreateChannels`], [`CanUseChannels`],
   and [`CanStreamReceiver`].

   When the stream terminates, the subscription also terminates.
*/
pub trait CanStreamSubscription {
    fn stream_subscription<S, T>(&self, stream: S) -> Arc<dyn Subscription<Item = T>>
    where
        S: Stream<Item = T> + Async,
        T: Async + Clone;
}

pub struct StreamSubscriptionTask<Runtime, S, T>
where
    Runtime: HasMutex + HasChannelTypes,
    T: Async,
    S: Stream<Item = T> + Async,
{
    pub stream: S,
    pub task_senders: Arc<Runtime::Mutex<Option<Vec<Runtime::Sender<T>>>>>,
}

#[async_trait]
impl<Runtime, S, T> Task for StreamSubscriptionTask<Runtime, S, T>
where
    Runtime: HasMutex + CanCreateChannels + CanUseChannels + CanStreamReceiver + HasBoxedStreamType,
    T: Clone + Async,
    S: Stream<Item = T> + Async,
{
    async fn run(self) {
        let task_senders = &self.task_senders;

        self.stream
            .for_each(|item| async move {
                let mut m_senders = Runtime::acquire_mutex(task_senders).await;

                if let Some(senders) = m_senders.deref_mut() {
                    // Remove senders where the receiver side has been dropped
                    senders.retain(|sender| Runtime::send(sender, item.clone()).is_ok());
                }
            })
            .await;

        let mut senders = Runtime::acquire_mutex(&self.task_senders).await;
        *senders = None;
    }
}

impl<Runtime> CanStreamSubscription for Runtime
where
    Runtime: CanSpawnTask
        + HasMutex
        + CanCreateChannels
        + CanUseChannels
        + CanStreamReceiver
        + HasBoxedStreamType,
{
    fn stream_subscription<S, T>(&self, stream: S) -> Arc<dyn Subscription<Item = T>>
    where
        S: Stream<Item = T> + Async,
        T: Async + Clone,
    {
        let stream_senders = Arc::new(Runtime::new_mutex(Some(Vec::new())));

        let task: StreamSubscriptionTask<Runtime, S, T> = StreamSubscriptionTask {
            stream,
            task_senders: stream_senders.clone(),
        };

        self.spawn_task(task);

        let subscription: MultiplexingSubscription<Runtime, T> =
            MultiplexingSubscription { stream_senders };

        Arc::new(subscription)
    }
}
