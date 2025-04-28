use hermes_core::chain_type_components::traits::HasAmountType;
use hermes_core::runtime_components::traits::{CanGenerateRandom, HasRuntime};
use hermes_core::test_components::chain_driver::traits::{
    HasChainType, RandomAmountGenerator, RandomAmountGeneratorComponent,
};
use hermes_prelude::*;

use crate::chain::types::Amount;

pub struct GenerateRandomAmount;

#[cgp_provider(RandomAmountGeneratorComponent)]
impl<ChainDriver, Chain> RandomAmountGenerator<ChainDriver> for GenerateRandomAmount
where
    ChainDriver: HasChainType<Chain = Chain> + HasRuntime + CanRaiseAsyncError<&'static str>,
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
