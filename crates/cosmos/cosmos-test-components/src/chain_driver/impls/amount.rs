use cgp_core::CanRaiseError;
use hermes_test_components::chain_driver::traits::fields::amount::{
    ProvideAmountMethods, RandomAmountGenerator,
};
use hermes_test_components::chain_driver::traits::types::amount::{
    AmountTypeProvider, HasAmountType,
};
use hermes_test_components::chain_driver::traits::types::denom::HasDenomType;
use rand::prelude::Rng;

use crate::chain_driver::types::amount::Amount;
use crate::chain_driver::types::denom::Denom;

pub struct ProvideU128AmountWithDenom;

impl<ChainDriver> AmountTypeProvider<ChainDriver> for ProvideU128AmountWithDenom
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
    ChainDriver: HasAmountType<Amount = Amount> + CanRaiseError<&'static str>,
{
    fn random_amount(min: usize, max: &Amount) -> Amount {
        let mut rng = rand::thread_rng();

        let max_quantity = max.quantity as usize;
        let quantity = rng.gen_range(min..max_quantity);

        Amount {
            quantity: quantity as u128,
            denom: max.denom.clone(),
        }
    }
}
