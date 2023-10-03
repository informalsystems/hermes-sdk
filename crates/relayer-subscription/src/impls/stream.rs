use alloc::sync::Arc;
use core::ops::DerefMut;

use cgp_core::traits::Async;
use futures_core::stream::Stream;
use futures_util::stream::StreamExt;
use ibc_relayer_components::runtime::traits::mutex::HasMutex;

use ibc_relayer_components_extra::runtime::traits::channel::{
    CanCreateChannels, CanStreamReceiver, CanUseChannels,
};
use ibc_relayer_components_extra::runtime::traits::spawn::CanSpawnTask;

use crate::impls::multiplex::MultiplexingSubscription;
use crate::std_prelude::*;
use crate::traits::stream::HasAsyncStreamType;
use crate::traits::subscription::Subscription;

/**
   Allows multiplexing of a single [`Stream`] into a subscription.
   This is an auto trait implemented by all runtime contexts that implement
   [`HasSpawner`], [`HasMutex`], [`CanCreateChannels`], [`CanUseChannels`],
   and [`CanStreamReceiver`].

   When the stream terminates, the subscription also terminates.
*/
pub trait CanStreamSubscription {
    fn stream_subscription<T>(
        &self,
        stream: impl Stream<Item = T> + Send + 'static,
    ) -> Arc<dyn Subscription<Item = T>>
    where
        T: Async + Clone;
}

impl<Runtime> CanStreamSubscription for Runtime
where
    Runtime: CanSpawnTask
        + HasMutex
        + CanCreateChannels
        + CanUseChannels
        + CanStreamReceiver
        + HasAsyncStreamType,
{
    fn stream_subscription<T>(
        &self,
        stream: impl Stream<Item = T> + Send + 'static,
    ) -> Arc<dyn Subscription<Item = T>>
    where
        T: Async + Clone,
    {
        let stream_senders = Arc::new(Runtime::new_mutex(Some(Vec::new())));

        let spawner = self.spawner();
        let task_senders = stream_senders.clone();

        spawner.spawn(async move {
            let task_senders = &task_senders;

            stream
                .for_each(|item| async move {
                    let mut m_senders = Runtime::acquire_mutex(task_senders).await;

                    if let Some(senders) = m_senders.deref_mut() {
                        // Remove senders where the receiver side has been dropped
                        senders.retain(|sender| Runtime::send(sender, item.clone()).is_ok());
                    }
                })
                .await;

            let mut senders = Runtime::acquire_mutex(task_senders).await;
            *senders = None;
        });

        let subscription: MultiplexingSubscription<Runtime, T> =
            MultiplexingSubscription { stream_senders };

        Arc::new(subscription)
    }
}
