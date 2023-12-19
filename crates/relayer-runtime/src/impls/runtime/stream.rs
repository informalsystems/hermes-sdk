use core::pin::Pin;

use cgp_core::Async;
use futures::stream::Stream;
use ibc_relayer_subscription::traits::stream::HasAsyncStreamType;

use crate::types::runtime::TokioRuntimeContext;

impl HasAsyncStreamType for TokioRuntimeContext {
    fn from_async_stream<T>(
        stream: Pin<Box<dyn Stream<Item = T> + Send + Sync + 'static>>,
    ) -> Self::Stream<T>
    where
        T: Async,
    {
        stream
    }

    fn to_async_stream<T>(
        stream: Self::Stream<T>,
    ) -> Pin<Box<dyn Stream<Item = T> + Send + Sync + 'static>>
    where
        T: Async,
    {
        stream
    }
}
