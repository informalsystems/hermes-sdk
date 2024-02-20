use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGenerator;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::random::CanGenerateRandom;

use crate::chain::types::amount::Amount;

pub struct GenerateRandomAmount;

impl<ChainDriver, Chain> RandomAmountGenerator<ChainDriver> for GenerateRandomAmount
where
    ChainDriver: HasChainType<Chain = Chain> + HasRuntime + CanRaiseError<&'static str>,
    ChainDriver::Runtime: CanGenerateRandom<u128>,
    Chain: HasAmountType<Amount = Amount>,
{
    async fn random_amount(chain_driver: &ChainDriver, min: usize, max: &Amount) -> Amount {
        let quantity = chain_driver
            .runtime()
            .random_range(min as u128, max.quantity)
            .await;

        Amount {
            quantity,
            denom: max.denom.clone(),
        }
    }
}
