use cgp::prelude::*;

use crate::traits::stream::HasStreamType;

#[cgp_component {
  name: SubscriptionComponent,
  provider: ProvideSubscription,
  context: Runtime,
}]
#[async_trait]
pub trait HasSubscription: HasStreamType {
    type Subscription<Item: Async>: Async;

    async fn subscribe<T>(subcription: &Self::Subscription<T>) -> Option<Self::Stream<T>>
    where
        T: Async;
}
