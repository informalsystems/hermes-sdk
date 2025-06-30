use hermes_core::chain_type_components::traits::{HasAmountType, HasDenomType};
use hermes_core::runtime_components::traits::{CanGenerateRandom, HasRuntime};
use hermes_core::test_components::chain_driver::traits::{
    HasChainType, RandomAmountGenerator, RandomAmountGeneratorComponent,
};
use hermes_prelude::*;

use crate::chain::types::{Amount, Denom};
pub struct GenerateRandomAmount;

#[cgp_provider(RandomAmountGeneratorComponent)]
impl<ChainDriver, Chain> RandomAmountGenerator<ChainDriver> for GenerateRandomAmount
where
    ChainDriver: HasChainType<Chain = Chain> + HasRuntime + CanRaiseAsyncError<&'static str>,
    ChainDriver::Runtime: CanGenerateRandom<u128>,
    Chain: HasAmountType<Amount = Amount> + HasDenomType<Denom = Denom>,
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

    async fn fixed_amount(_chain_driver: &ChainDriver, amount: usize, denom: &Denom) -> Amount {
        Amount {
            quantity: amount as u128,
            denom: denom.clone(),
        }
    }
}
