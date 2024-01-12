use cgp_core::prelude::*;

use crate::driver::traits::types::chain::HasChainType;
use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};
use crate::types::index::Index;

#[derive_component(BootstrapAtComponent, ProvideBootstrapAt<Setup>)]
pub trait HasBootstrapAt<const I: usize>: HasChainTypeAt<I> {
    type Bootstrap: HasChainType<Chain = ChainTypeAt<Self, I>>;

    fn chain_bootstrap(&self, index: Index<I>) -> &Self::Bootstrap;
}

pub type BootstrapAt<Context, const I: usize> = <Context as HasBootstrapAt<I>>::Bootstrap;
