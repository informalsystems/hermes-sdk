use cgp_core::Async;

pub trait HasTwoChains: Async {
    type ChainA: Async;

    type ChainB: Async;
}
