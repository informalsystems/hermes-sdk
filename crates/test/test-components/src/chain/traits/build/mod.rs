use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainId;

use crate::driver::traits::types::chain::HasChainType;

#[derive_component(ChainIdFromStringBuilderComponent, ChainIdFromStringBuilder<Chain>)]
pub trait CanBuildChainIdFromString: HasChainType
where
    Self::Chain: HasChainIdType,
{
    fn build_chain_id_from_string(chain_id: &str) -> ChainId<Self::Chain>;
}
