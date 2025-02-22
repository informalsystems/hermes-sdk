use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_test_components::chain::traits::chain_id::CanBuildChainIdFromString;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::generator::generate_chain_id::{
    ChainIdGenerator, ChainIdGeneratorComponent,
};

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
