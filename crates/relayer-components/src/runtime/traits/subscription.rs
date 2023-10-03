use async_trait::async_trait;
use cgp_core::traits::Async;

use crate::runtime::traits::stream::HasStreamType;
use crate::std_prelude::*;

#[async_trait]
pub trait HasSubscriptionType: HasStreamType {
    type Subscription<Item: Async>: Async;

    async fn subscribe<T>(subcription: &Self::Subscription<T>) -> Option<Self::Stream<T>>
    where
        T: Async;
}
