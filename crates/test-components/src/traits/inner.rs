use cgp_core::Async;

pub trait HasInner: Async {
    type Inner: Async;

    fn inner(&self) -> &Self::Inner;
}
