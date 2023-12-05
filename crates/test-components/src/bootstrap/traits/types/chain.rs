use cgp_core::prelude::*;

pub trait HasChainType: Async {
    type Chain: Async;
}
