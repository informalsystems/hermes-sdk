use cgp::prelude::*;

#[cgp_component {
  name: StreamTypeComponent,
  provider: ProvideStreamType,
  context: Runtime,
}]
pub trait HasStreamType: Async {
    type Stream<Item: Async>: Async;
}

#[cgp_component {
  name: StreamMapperComponent,
  provider: StreamMapper,
  context: Runtime,
}]
pub trait CanMapStream: HasStreamType {
    fn map_stream<T, U, M>(stream: Self::Stream<T>, mapper: M) -> Self::Stream<U>
    where
        T: Async,
        U: Async,
        M: Fn(T) -> U + Async;
}
