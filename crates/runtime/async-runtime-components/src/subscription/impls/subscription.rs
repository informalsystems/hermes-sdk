use alloc::sync::Arc;

use cgp::prelude::*;
use hermes_runtime_components::traits::subscription::ProvideSubscription;

use crate::stream::traits::boxed::HasBoxedStreamType;
use crate::subscription::traits::subscription::Subscription;

pub struct ProvideBoxedSubscription;

impl<Runtime> ProvideSubscription<Runtime> for ProvideBoxedSubscription
where
    Runtime: HasBoxedStreamType,
{
    type Subscription<Item: Async> = Arc<dyn Subscription<Item = Item>>;

    async fn subscribe<T>(subscription: &Self::Subscription<T>) -> Option<Runtime::Stream<T>>
    where
        T: Async,
    {
        subscription
            .subscribe()
            .await
            .map(Runtime::from_boxed_stream)
    }
}
