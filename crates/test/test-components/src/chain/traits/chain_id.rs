use hermes_prelude::*;
use hermes_relayer_components::chain::traits::HasChainIdType;

#[cgp_component {
  provider: ChainIdFromStringBuilder,
  context: Chain,
}]
pub trait CanBuildChainIdFromString: HasChainIdType {
    fn build_chain_id_from_string(chain_id: &str) -> Self::ChainId;
}
