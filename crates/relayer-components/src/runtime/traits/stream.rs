use cgp_core::traits::Async;

pub trait HasStreamType: Async {
    type Stream<Item: Async>: Async;
}
