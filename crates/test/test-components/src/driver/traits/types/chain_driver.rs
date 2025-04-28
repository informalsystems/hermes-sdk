use hermes_prelude::*;

#[cgp_type]
pub trait HasChainDriverType: Async {
    type ChainDriver: Async;
}
