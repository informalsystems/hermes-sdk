use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainIdOf;

use crate::chain_driver::traits::types::chain::HasChainType;

#[derive_component(ChainIdFromStringBuilderComponent, ChainIdFromStringBuilder<Chain>)]
pub trait CanBuildChainIdFromString: HasChainIdType {
    fn build_chain_id_from_string(chain_id: &str) -> Self::ChainId;
}
