use cgp_core::prelude::*;

use crate::runtime::traits::stream::HasStreamType;

#[derive_component(SubscriptionComponent, ProvideSubscription<Runtime>)]
#[async_trait]
pub trait HasSubscription: HasStreamType {
    type Subscription<Item: Async>: Async;

    async fn subscribe<T>(subcription: &Self::Subscription<T>) -> Option<Self::Stream<T>>
    where
        T: Async;
}
