use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ops::DerefMut;
use core::pin::Pin;

use cgp_core::prelude::*;
use futures_core::stream::Stream;
use futures_util::stream::StreamExt;
use ibc_relayer_components::runtime::traits::mutex::HasMutex;
use ibc_relayer_components::runtime::traits::stream::HasStreamType;
use ibc_relayer_components::runtime::traits::task::Task;
use ibc_relayer_components_extra::runtime::traits::channel::{
    CanCreateChannels, CanStreamReceiver, CanUseChannels, HasChannelTypes,
};
use ibc_relayer_components_extra::runtime::traits::spawn::CanSpawnTask;

use crate::stream::traits::boxed::HasBoxedStreamType;
use crate::subscription::traits::subscription::Subscription;

/**
   Multiplex the incoming [`Stream`] provided by an underlying [`Subscription`]
   into multiple outgoing [`Stream`]s through a background task. This is
   an auto trait implemented by all runtime contexts that implement
   [`HasSpawner`], [`HasMutex`], [`CanCreateChannels`], [`CanUseChannels`],
   and [`CanStreamReceiver`].

   This can be used to improve the efficiency of naive subscriptions created from
   [`CanCreateClosureSubscription`](ibc_relayer_components::runtime::impls::subscription::closure::CanCreateClosureSubscription).
   For example, one can first create a subscription closure that establishes
   new network connection each time `subscribe` is called. The subscription
   closure is then passed to [`multiplex_subscription`](Self::multiplex_subscription),
   which would return a wrapped subscription which would only create one
   network connection at a time.

   The multiplexed subscription also attempts to recover by calling the
   [`subscribe`](Subscription::subscribe) method of the underlying subsciption
   again, if a given [`Stream`] terminates. This would allow for auto recovery
   from underlying errors such as network disconnection. The multiplexed
   subscription would only terminate if the underlying
   [`subscribe`](Subscription::subscribe) returns `None`.

   The streams returned from the [`subscribe`](Subscription::subscribe) of
   the multiplexed subscription will automatically resume streaming from
   a new underlying stream, if the original underlying stream is terminated.
   However, since a consumer cannot know if a [`Subscription`] implementation
   is multiplexed or not, it should always retry calling
   [`subscribe`](Subscription::subscribe) in case a [`Stream`] ends.
*/
pub trait CanMultiplexSubscription {
    /**
       Multiplex a given subscription, with a mapper function that maps the
       item coming from the underlying subscription from `T` to `U`. Returns
       a new multiplexed subscription that shares the same underlying [`Stream`].
    */
    fn multiplex_subscription<T, U>(
        &self,
        subscription: impl Subscription<Item = T>,
        map_item: impl Fn(T) -> U + Async,
    ) -> Arc<dyn Subscription<Item = U>>
    where
        T: Async + Clone,
        U: Async + Clone;
}

pub struct MultiplexSubscriptionTask<Runtime, S, M, T, U>
where
    Runtime: HasMutex + HasChannelTypes,
    S: Subscription<Item = T>,
    M: Fn(T) -> U + Async,
    T: Async,
    U: Async,
{
    pub subscription: S,
    pub mapper: M,
    pub task_senders: Arc<Runtime::Mutex<Option<Vec<Runtime::Sender<U>>>>>,
    pub phantom: PhantomData<Runtime>,
}

#[async_trait]
impl<Runtime, S, M, T, U> Task for MultiplexSubscriptionTask<Runtime, S, M, T, U>
where
    Runtime: HasMutex + CanUseChannels,
    S: Subscription<Item = T>,
    M: Fn(T) -> U + Async,
    T: Async,
    U: Async + Clone,
{
    async fn run(self) {
        loop {
            let m_stream = self.subscription.subscribe().await;

            match m_stream {
                Some(stream) => {
                    let task_senders = &self.task_senders;
                    let map_item = &self.mapper;

                    stream
                        .for_each(|item| async move {
                            let mapped = map_item(item);
                            let mut m_senders = Runtime::acquire_mutex(task_senders).await;

                            if let Some(senders) = m_senders.deref_mut() {
                                let mut new_senders = Vec::new();

                                for sender in senders.drain(..) {
                                    let send_result = Runtime::send(&sender, mapped.clone()).await;
                                    // Remove senders where the receiver side has been dropped,
                                    // i.e. keep the ones where sending is successful
                                    if send_result.is_ok() {
                                        new_senders.push(sender);
                                    }
                                }

                                *senders = new_senders;
                            }
                        })
                        .await;
                }
                None => {
                    // If the underlying subscription returns `None` from `subscribe`, clears the senders
                    // queue inside the mutex and set it to `None` and return. This will cause subsequent
                    // calls to `subscribe` for the multiplexed subscription to also return `None`.
                    let mut senders = Runtime::acquire_mutex(&self.task_senders).await;
                    *senders = None;
                    return;
                }
            }
        }
    }
}

impl<Runtime> CanMultiplexSubscription for Runtime
where
    Runtime: CanSpawnTask
        + HasMutex
        + CanCreateChannels
        + CanUseChannels
        + CanStreamReceiver
        + HasBoxedStreamType,
{
    fn multiplex_subscription<T, U>(
        &self,
        subscription: impl Subscription<Item = T>,
        mapper: impl Fn(T) -> U + Async,
    ) -> Arc<dyn Subscription<Item = U>>
    where
        T: Async + Clone,
        U: Async + Clone,
    {
        let stream_senders = Arc::new(Runtime::new_mutex(Some(Vec::new())));

        let task = MultiplexSubscriptionTask {
            subscription,
            mapper,
            task_senders: stream_senders.clone(),
            phantom: PhantomData::<Runtime>,
        };

        self.spawn_task(task);

        let subscription: MultiplexingSubscription<Runtime, U> =
            MultiplexingSubscription { stream_senders };

        Arc::new(subscription)
    }
}

type StreamSender<Runtime, T> = <Runtime as HasChannelTypes>::Sender<T>;

type StreamSenders<Runtime, T> =
    Arc<<Runtime as HasMutex>::Mutex<Option<Vec<StreamSender<Runtime, T>>>>>;

pub struct MultiplexingSubscription<Runtime, T>
where
    T: Async,
    Runtime: HasChannelTypes + HasMutex,
{
    pub stream_senders: StreamSenders<Runtime, T>,
}

impl<Runtime, T> Clone for MultiplexingSubscription<Runtime, T>
where
    T: Async,
    Runtime: HasChannelTypes
        + HasMutex
        + HasStreamType<Stream<T> = Pin<Box<dyn Stream<Item = T> + Send + Sync + 'static>>>,
{
    fn clone(&self) -> Self {
        Self {
            stream_senders: self.stream_senders.clone(),
        }
    }
}

#[async_trait]
impl<Runtime, T> Subscription for MultiplexingSubscription<Runtime, T>
where
    T: Async,
    Runtime: HasMutex + CanCreateChannels + CanStreamReceiver + HasBoxedStreamType,
{
    type Item = T;

    async fn subscribe(&self) -> Option<Pin<Box<dyn Stream<Item = T> + Send + Sync + 'static>>>
    where
        T: Async,
    {
        let mut m_senders = Runtime::acquire_mutex(&self.stream_senders).await;

        match m_senders.as_mut() {
            Some(senders) => {
                let (sender, receiver) = Runtime::new_channel();
                senders.push(sender);

                let stream = Runtime::receiver_to_stream(receiver);

                Some(Runtime::to_boxed_stream(stream))
            }
            None => None,
        }
    }
}
