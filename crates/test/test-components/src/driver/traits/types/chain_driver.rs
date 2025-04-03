use cgp::prelude::*;

#[cgp_type]
pub trait HasChainDriverType: Async {
    type ChainDriver: Async;
}
