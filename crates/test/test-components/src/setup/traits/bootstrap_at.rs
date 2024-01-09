use cgp_core::prelude::*;

use crate::bootstrap::traits::types::chain::HasChainType;
use crate::driver::traits::types::chain::{ChainTypeAt, HasChainTypeAt};

#[derive_component(ChainBootstrapAtComponent, ProvideChainBootstrapAt<Setup>)]
pub trait HasChainBootstrapAt<const I: usize>: HasChainTypeAt<I> {
    type Bootstrap: HasChainType<Chain = ChainTypeAt<Self, I>>;

    fn chain_bootstrap(&self) -> &Self::Bootstrap;
}
