use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::test_components::chain::traits::{
    ChainIdFromStringBuilder, ChainIdFromStringBuilderComponent,
};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::ChainId;

pub struct BuildCosmosChainIdFromString;

#[cgp_provider(ChainIdFromStringBuilderComponent)]
impl<Chain> ChainIdFromStringBuilder<Chain> for BuildCosmosChainIdFromString
where
    Chain: HasChainIdType<ChainId = ChainId>,
{
    fn build_chain_id_from_string(chain_id: &str) -> ChainId {
        ChainId::new(chain_id).unwrap()
    }
}
