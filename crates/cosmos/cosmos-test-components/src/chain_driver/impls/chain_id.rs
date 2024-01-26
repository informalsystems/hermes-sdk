use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_test_components::chain_driver::traits::build::chain_id::ChainIdFromStringBuilder;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

pub struct BuildCosmosChainIdFromString;

impl<ChainDriver> ChainIdFromStringBuilder<ChainDriver> for BuildCosmosChainIdFromString
where
    ChainDriver: HasChainType,
    ChainDriver::Chain: HasChainIdType<ChainId = ChainId>,
{
    fn build_chain_id_from_string(chain_id: &str) -> ChainId {
        ChainId::from_string(chain_id)
    }
}
