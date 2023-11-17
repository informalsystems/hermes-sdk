use cgp_core::Async;

pub trait HasInnerType: Async {
    type Inner: Async;
}

pub trait HasInner: HasInnerType {
    fn inner(&self) -> &Self::Inner;
}
