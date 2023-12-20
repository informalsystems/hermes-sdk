use cgp_core::prelude::*;
use hermes_test_components::bootstrap::traits::types::chain::HasChainType;
use hermes_test_components::chain::traits::build::CanBuildChainIdFromString;
use rand::prelude::*;

use crate::bootstrap::traits::fields::random_id::HasRandomIdFlag;
use crate::bootstrap::traits::generator::generate_chain_id::ChainIdGenerator;

pub struct GenerateRandomChainId;

#[async_trait]
impl<Bootstrap, Chain> ChainIdGenerator<Bootstrap> for GenerateRandomChainId
where
    Bootstrap: HasChainType<Chain = Chain> + HasRandomIdFlag,
    Chain: CanBuildChainIdFromString,
{
    async fn generate_chain_id(bootstrap: &Bootstrap, chain_id_prefix: &str) -> Chain::ChainId {
        if bootstrap.should_randomize_identifiers() {
            let postfix: u32 = {
                let mut rng = thread_rng();
                rng.gen()
            };

            let chain_id = format!("{chain_id_prefix}-{postfix}");

            Chain::build_chain_id_from_string(&chain_id)
        } else {
            Chain::build_chain_id_from_string(chain_id_prefix)
        }
    }
}
