use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::fields::amount::{
    ProvideAmountMethods, RandomAmountGenerator,
};
use hermes_test_components::chain_driver::traits::types::amount::{
    HasAmountType, ProvideAmountType,
};
use hermes_test_components::chain_driver::traits::types::denom::HasDenomType;
use hermes_test_components::runtime::traits::random::CanGenerateRandom;

use crate::chain_driver::types::amount::Amount;
use crate::chain_driver::types::denom::Denom;

pub struct ProvideU128AmountWithDenom;

impl<ChainDriver> ProvideAmountType<ChainDriver> for ProvideU128AmountWithDenom
where
    ChainDriver: HasDenomType<Denom = Denom>,
{
    type Amount = Amount;

    fn amount_denom(amount: &Amount) -> &<ChainDriver as HasDenomType>::Denom {
        &amount.denom
    }
}

impl<ChainDriver> ProvideAmountMethods<ChainDriver> for ProvideU128AmountWithDenom
where
    ChainDriver: HasAmountType<Amount = Amount> + CanRaiseError<&'static str>,
{
    fn add_amount(current: &Amount, amount: &Amount) -> Result<Amount, ChainDriver::Error> {
        if current.denom != amount.denom {
            return Err(ChainDriver::raise_error("mismatch denom"));
        }

        let quantity = current
            .quantity
            .checked_add(amount.quantity)
            .ok_or_else(|| ChainDriver::raise_error("overflow adding amount"))?;

        Ok(Amount {
            quantity,
            denom: current.denom.clone(),
        })
    }

    fn subtract_amount(current: &Amount, amount: &Amount) -> Result<Amount, ChainDriver::Error> {
        if current.denom != amount.denom {
            return Err(ChainDriver::raise_error("mismatch denom"));
        }

        let quantity = current
            .quantity
            .checked_sub(amount.quantity)
            .ok_or_else(|| ChainDriver::raise_error("underflow subtracting amount"))?;

        Ok(Amount {
            quantity,
            denom: current.denom.clone(),
        })
    }
}

impl<ChainDriver> RandomAmountGenerator<ChainDriver> for ProvideU128AmountWithDenom
where
    ChainDriver: HasAmountType<Amount = Amount> + HasRuntime + CanRaiseError<&'static str>,
    ChainDriver::Runtime: CanGenerateRandom<u128>,
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
