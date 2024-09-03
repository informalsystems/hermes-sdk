use alloc::boxed::Box;
use core::marker::PhantomData;
use core::pin::Pin;

use cgp::prelude::*;
use futures_core::stream::Stream;

use crate::subscription::traits::subscription::Subscription;

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

#[async_trait::async_trait]
impl<T: Async> Subscription for EmptySubscription<T> {
    type Item = T;

    async fn subscribe(
        &self,
    ) -> Option<Pin<Box<dyn Stream<Item = Self::Item> + Send + Sync + 'static>>> {
        None
    }
}
