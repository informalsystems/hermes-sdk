use cgp_core::prelude::*;

use crate::driver::traits::types::chain_driver::HasChainDriverType;
use crate::driver::traits::types::chain_driver_at::{ChainDriverTypeAt, HasChainDriverTypeAt};
use crate::types::index::Index;

#[derive_component(BootstrapAtComponent, ProvideBootstrapAt<Setup>)]
pub trait HasBootstrapAt<const I: usize>: HasChainDriverTypeAt<I> {
    type Bootstrap: HasChainDriverType<ChainDriver = ChainDriverTypeAt<Self, I>>;

    fn chain_bootstrap(&self, index: Index<I>) -> &Self::Bootstrap;
}

pub type BootstrapAt<Context, const I: usize> = <Context as HasBootstrapAt<I>>::Bootstrap;
