use core::pin::Pin;

use cgp_core::traits::Async;
use futures::stream::Stream;
use futures::StreamExt;
use ibc_relayer_components::runtime::traits::stream::{CanMapStream, HasStreamType};

use crate::types::runtime::TokioRuntimeContext;

impl HasStreamType for TokioRuntimeContext {
    type Stream<Item: Async> = Pin<Box<dyn Stream<Item = Item> + Send + Sync + 'static>>;
}

impl CanMapStream for TokioRuntimeContext {
    fn map_stream<T, U, M>(stream: Self::Stream<T>, mapper: M) -> Self::Stream<U>
    where
        T: Async,
        U: Async,
        M: Fn(T) -> U + Async,
    {
        let mapped = stream.map(mapper);

        Box::pin(mapped)
    }
}
