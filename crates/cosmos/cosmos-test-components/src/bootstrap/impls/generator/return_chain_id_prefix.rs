use cgp::prelude::*;
use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::test_components::chain::traits::CanBuildChainIdFromString;
use hermes_core::test_components::chain_driver::traits::HasChainType;

use crate::bootstrap::traits::{ChainIdGenerator, ChainIdGeneratorComponent};

pub struct ReturnPrefixAsChainId;

#[cgp_provider(ChainIdGeneratorComponent)]
impl<Bootstrap, Chain> ChainIdGenerator<Bootstrap> for ReturnPrefixAsChainId
where
    Bootstrap: HasChainType<Chain = Chain>,
    Chain: HasChainIdType + CanBuildChainIdFromString,
{
    async fn generate_chain_id(_bootstrap: &Bootstrap, chain_id_prefix: &str) -> Chain::ChainId {
        Chain::build_chain_id_from_string(chain_id_prefix)
    }
}
