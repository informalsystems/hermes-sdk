use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::build::chain_id::CanBuildChainIdFromString;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::random::CanGenerateRandom;

use crate::bootstrap::traits::fields::random_id::HasRandomIdFlag;
use crate::bootstrap::traits::generator::generate_chain_id::ChainIdGenerator;

pub struct GenerateRandomChainId;

#[async_trait]
impl<Bootstrap, Chain, ChainDriver, Runtime> ChainIdGenerator<Bootstrap> for GenerateRandomChainId
where
    Bootstrap: HasChainDriverType<Chain = Chain, ChainDriver = ChainDriver>
        + HasRuntime<Runtime = Runtime>
        + HasRandomIdFlag,
    ChainDriver: CanBuildChainIdFromString<Chain = Chain>,
    Chain: HasChainIdType,
    Runtime: CanGenerateRandom<u32>,
{
    async fn generate_chain_id(bootstrap: &Bootstrap, chain_id_prefix: &str) -> Chain::ChainId {
        if bootstrap.should_randomize_identifiers() {
            let postfix = bootstrap.runtime().generate_random().await;

            let chain_id = format!("{chain_id_prefix}-{postfix}");

            ChainDriver::build_chain_id_from_string(&chain_id)
        } else {
            ChainDriver::build_chain_id_from_string(chain_id_prefix)
        }
    }
}
