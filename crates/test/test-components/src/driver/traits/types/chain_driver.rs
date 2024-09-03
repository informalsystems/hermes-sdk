use cgp::prelude::*;

use crate::chain_driver::traits::types::chain::HasChainType;

#[derive_component(ChainDriverTypeComponent, ProvideChainDriverType<Context>)]
pub trait HasChainDriverType: HasChainType {
    type ChainDriver: HasChainType;
}
