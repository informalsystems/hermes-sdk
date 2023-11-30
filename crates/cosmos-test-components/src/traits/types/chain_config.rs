use cgp_core::prelude::*;

pub trait HasChainConfigType: Async {
    type ChainConfig: Async;
}
