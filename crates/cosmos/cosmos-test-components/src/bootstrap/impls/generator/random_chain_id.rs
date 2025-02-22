use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_runtime_components::traits::random::CanGenerateRandom;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::chain_id::CanBuildChainIdFromString;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::fields::random_id::HasRandomIdFlag;
use crate::bootstrap::traits::generator::generate_chain_id::{
    ChainIdGenerator, ChainIdGeneratorComponent,
};

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
