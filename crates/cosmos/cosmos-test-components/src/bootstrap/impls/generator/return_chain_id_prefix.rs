use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_test_components::chain::traits::build::CanBuildChainIdFromString;

use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::generator::generate_chain_id::ChainIdGenerator;

pub struct ReturnPrefixAsChainId;

#[async_trait]
impl<Bootstrap, Chain, ChainDriver> ChainIdGenerator<Bootstrap> for ReturnPrefixAsChainId
where
    Bootstrap: HasChainDriverType<Chain = Chain, ChainDriver = ChainDriver>,
    ChainDriver: CanBuildChainIdFromString<Chain = Chain>,
    Chain: HasChainIdType,
{
    async fn generate_chain_id(_bootstrap: &Bootstrap, chain_id_prefix: &str) -> Chain::ChainId {
        ChainDriver::build_chain_id_from_string(chain_id_prefix)
    }
}
