use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;

#[cgp_component {
  name: ChainIdFromStringBuilderComponent,
  provider: ChainIdFromStringBuilder,
  context: Chain,
}]
pub trait CanBuildChainIdFromString: HasChainIdType {
    fn build_chain_id_from_string(chain_id: &str) -> Self::ChainId;
}
