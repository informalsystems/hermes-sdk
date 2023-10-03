use cgp_core::Async;

pub trait HasStreamType: Async {
    type Stream<Item: Async>: Async;
}

pub trait CanMapStream: HasStreamType {
    fn map_stream<T, U, M>(stream: Self::Stream<T>, mapper: M) -> Self::Stream<U>
    where
        T: Async,
        U: Async,
        M: Fn(T) -> U + Async;
}
