use core::pin::Pin;

use cgp_core::Async;
use futures_core::stream::Stream;
use hermes_runtime_components::traits::stream::HasStreamType;

pub trait HasAsyncStreamType: HasStreamType {
    fn from_async_stream<T>(
        stream: Pin<Box<dyn Stream<Item = T> + Send + Sync + 'static>>,
    ) -> Self::Stream<T>
    where
        T: Async;

    fn to_async_stream<T>(
        stream: Self::Stream<T>,
    ) -> Pin<Box<dyn Stream<Item = T> + Send + Sync + 'static>>
    where
        T: Async;
}
