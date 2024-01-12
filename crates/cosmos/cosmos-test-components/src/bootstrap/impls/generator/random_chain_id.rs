use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_test_components::chain::traits::build::CanBuildChainIdFromString;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use rand::prelude::*;

use crate::bootstrap::traits::fields::random_id::HasRandomIdFlag;
use crate::bootstrap::traits::generator::generate_chain_id::ChainIdGenerator;

pub struct GenerateRandomChainId;

#[async_trait]
impl<Bootstrap, Chain, ChainDriver> ChainIdGenerator<Bootstrap> for GenerateRandomChainId
where
    Bootstrap: HasChainDriverType<Chain = Chain, ChainDriver = ChainDriver> + HasRandomIdFlag,
    ChainDriver: CanBuildChainIdFromString<Chain = Chain>,
    Chain: HasChainIdType,
{
    async fn generate_chain_id(bootstrap: &Bootstrap, chain_id_prefix: &str) -> Chain::ChainId {
        if bootstrap.should_randomize_identifiers() {
            let postfix: u32 = {
                let mut rng = thread_rng();
                rng.gen()
            };

            let chain_id = format!("{chain_id_prefix}-{postfix}");

            ChainDriver::build_chain_id_from_string(&chain_id)
        } else {
            ChainDriver::build_chain_id_from_string(chain_id_prefix)
        }
    }
}
