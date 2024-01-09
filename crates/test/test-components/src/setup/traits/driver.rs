use cgp_core::Async;

pub trait HasDriverType: Async {
    type Driver: Async;
}
