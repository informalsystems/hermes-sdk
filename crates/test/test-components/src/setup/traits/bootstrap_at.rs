use cgp_core::prelude::*;

use crate::bootstrap::traits::types::chain::HasChainType;
use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};
use crate::setup::traits::driver::HasDriverType;

#[derive_component(ChainBootstrapAtComponent, ProvideChainBootstrapAt<Setup>)]
pub trait HasBootstrapAt<const I: usize>: HasDriverType
where
    Self::Driver: HasChainTypeAt<I>,
{
    type Bootstrap: HasChainType<Chain = ChainTypeAt<Self::Driver, I>>;

    fn chain_bootstrap(&self) -> &Self::Bootstrap;
}

pub type BootstrapAt<Context, const I: usize> = <Context as HasBootstrapAt<I>>::Bootstrap;
