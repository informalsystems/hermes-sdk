use cgp::core::error::CanRaiseAsyncError;
use hermes_test_components::chain::traits::types::amount::{
    HasAmountType, ProvideAmountMethods, ProvideAmountType,
};
use hermes_test_components::chain::traits::types::denom::HasDenomType;

use crate::chain::types::amount::Amount;
use crate::chain::types::denom::Denom;

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
    ChainDriver: HasAmountType<Amount = Amount> + CanRaiseAsyncError<&'static str>,
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
