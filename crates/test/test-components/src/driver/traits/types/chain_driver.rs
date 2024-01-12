use cgp_core::prelude::*;

use crate::driver::traits::types::chain::HasChainType;

#[derive_component(ChainDriverTypeComponent, ProvideChainDriverType<Context>)]
pub trait HasChainDriverType: HasChainType {
    type ChainDriver: HasChainType<Chain = Self::Chain>;
}
