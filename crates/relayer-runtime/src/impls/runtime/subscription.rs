use alloc::sync::Arc;
use core::pin::Pin;

use async_trait::async_trait;
use cgp_core::Async;
use futures::stream::Stream;
use ibc_relayer_components::runtime::traits::subscription::HasSubscriptionType;
use ibc_relayer_subscription::traits::subscription::Subscription;

use crate::types::runtime::TokioRuntimeContext;

#[async_trait]
impl HasSubscriptionType for TokioRuntimeContext {
    type Subscription<Item: Async> = Arc<dyn Subscription<Item = Item>>;

    async fn subscribe<T>(
        subscription: &Self::Subscription<T>,
    ) -> Option<Pin<Box<dyn Stream<Item = T> + Send + Sync + 'static>>>
    where
        T: Async,
    {
        subscription.subscribe().await
    }
}
