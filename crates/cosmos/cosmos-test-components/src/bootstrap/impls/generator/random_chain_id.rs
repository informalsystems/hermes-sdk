use cgp::prelude::*;
use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::runtime_components::traits::{CanGenerateRandom, HasRuntime};
use hermes_core::test_components::chain::traits::CanBuildChainIdFromString;
use hermes_core::test_components::chain_driver::traits::HasChainType;

use crate::bootstrap::traits::{ChainIdGenerator, ChainIdGeneratorComponent, HasRandomIdFlag};

pub struct GenerateRandomChainId;

#[cgp_provider(ChainIdGeneratorComponent)]
impl<Bootstrap, Chain, Runtime> ChainIdGenerator<Bootstrap> for GenerateRandomChainId
where
    Bootstrap: HasChainType<Chain = Chain> + HasRuntime<Runtime = Runtime> + HasRandomIdFlag,
    Chain: HasChainIdType + CanBuildChainIdFromString,
    Runtime: CanGenerateRandom<u32>,
{
    async fn generate_chain_id(bootstrap: &Bootstrap, chain_id_prefix: &str) -> Chain::ChainId {
        if bootstrap.should_randomize_identifiers() {
            let postfix = bootstrap.runtime().generate_random().await;

            let chain_id = format!("{chain_id_prefix}-{postfix}");

            Chain::build_chain_id_from_string(&chain_id)
        } else {
            Chain::build_chain_id_from_string(chain_id_prefix)
        }
    }
}
