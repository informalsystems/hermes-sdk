use cgp_core::Async;

pub trait HasTokenType {
    type Token: Async;
}
