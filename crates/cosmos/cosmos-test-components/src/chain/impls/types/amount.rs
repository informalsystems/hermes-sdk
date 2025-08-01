use hermes_core::chain_components::traits::{
    AmountQuantityGetter, AmountQuantityGetterComponent, HasQuantityType,
};
use hermes_core::chain_type_components::traits::{
    AmountDenomGetter, AmountDenomGetterComponent, AmountTypeProviderComponent, HasAmountType,
    HasDenomType,
};
use hermes_core::test_components::chain::traits::{AmountMethodsComponent, ProvideAmountMethods};
use hermes_prelude::*;

use crate::chain::types::{Amount, Denom};

pub struct UseCosmosAmount;

delegate_components! {
    UseCosmosAmount {
        AmountTypeProviderComponent: UseType<Amount>
    }
}

#[cgp_provider(AmountDenomGetterComponent)]
impl<Chain> AmountDenomGetter<Chain> for UseCosmosAmount
where
    Chain: HasAmountType<Amount = Amount> + HasDenomType<Denom = Denom>,
{
    fn amount_denom(amount: &Amount) -> &Denom {
        &amount.denom
    }
}

#[cgp_provider(AmountQuantityGetterComponent)]
impl<Chain> AmountQuantityGetter<Chain> for UseCosmosAmount
where
    Chain: HasAmountType<Amount = Amount> + HasQuantityType<Quantity = u128>,
{
    fn amount_quantity(amount: &Amount) -> &u128 {
        &amount.quantity
    }
}

#[cgp_provider(AmountMethodsComponent)]
impl<ChainDriver> ProvideAmountMethods<ChainDriver> for UseCosmosAmount
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
