use core::marker::PhantomData;
use core::pin::Pin;

use cgp_async::async_generic_trait;
use cgp_core::traits::Async;
use futures_core::stream::Stream;

use crate::runtime::traits::subscription::Subscription;
use crate::std_prelude::*;

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

#[async_generic_trait]
impl<T: Async> Subscription for EmptySubscription<T> {
    type Item = T;

    async fn subscribe(&self) -> Option<Pin<Box<dyn Stream<Item = Self::Item> + Send + 'static>>> {
        None
    }
}
