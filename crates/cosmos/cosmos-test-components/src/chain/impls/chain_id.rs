use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_test_components::chain::traits::chain_id::ChainIdFromStringBuilder;
use ibc::core::host::types::identifiers::ChainId;

pub struct BuildCosmosChainIdFromString;

impl<Chain> ChainIdFromStringBuilder<Chain> for BuildCosmosChainIdFromString
where
    Chain: HasChainIdType<ChainId = ChainId>,
{
    fn build_chain_id_from_string(chain_id: &str) -> ChainId {
        ChainId::new(chain_id).unwrap()
    }
}
