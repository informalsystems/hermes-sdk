use core::marker::PhantomData;
use core::pin::Pin;

use async_trait::async_trait;
use cgp_core::traits::Async;
use futures::stream::Stream;

use crate::traits::subscription::Subscription;

pub struct EmptySubscription<T>(pub PhantomData<T>);

impl<T> Default for EmptySubscription<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> EmptySubscription<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

#[async_trait]
impl<T: Async> Subscription for EmptySubscription<T> {
    type Item = T;

    async fn subscribe(
        &self,
    ) -> Option<Pin<Box<dyn Stream<Item = Self::Item> + Send + Sync + 'static>>> {
        None
    }
}
