use cgp_core::prelude::*;

#[derive_component(StreamTypeComponent, ProvideStreamType<Runtime>)]
pub trait HasStreamType: Async {
    type Stream<Item: Async>: Async;
}

#[derive_component(StreamMapperComponent, StreamMapper<Runtime>)]
pub trait CanMapStream: HasStreamType {
    fn map_stream<T, U, M>(stream: Self::Stream<T>, mapper: M) -> Self::Stream<U>
    where
        T: Async,
        U: Async,
        M: Fn(T) -> U + Async;
}
