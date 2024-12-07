use cgp::prelude::*;

use crate::chain_driver::traits::types::chain::HasChainType;

#[cgp_component {
  name: ChainDriverTypeComponent,
  provider: ProvideChainDriverType,
}]
pub trait HasChainDriverType: HasChainType {
    type ChainDriver: HasChainType;
}
