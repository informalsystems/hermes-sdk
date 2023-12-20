use cgp_core::prelude::*;
use futures_util::stream::StreamExt;
use ibc_relayer_components::runtime::traits::stream::StreamMapper;

use crate::stream::traits::boxed::HasBoxedStreamType;

pub struct BoxedStreamMapper;

impl<Runtime> StreamMapper<Runtime> for BoxedStreamMapper
where
    Runtime: HasBoxedStreamType,
{
    fn map_stream<T, U, M>(stream: Runtime::Stream<T>, mapper: M) -> Runtime::Stream<U>
    where
        T: Async,
        U: Async,
        M: Fn(T) -> U + Async,
    {
        let mapped = Runtime::to_boxed_stream(stream).map(mapper);

        Runtime::from_boxed_stream(Box::pin(mapped))
    }
}
