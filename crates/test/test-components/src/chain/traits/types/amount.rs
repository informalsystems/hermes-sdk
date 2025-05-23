use hermes_chain_type_components::traits::HasAmountType;
use hermes_prelude::*;

#[cgp_component {
  name: AmountMethodsComponent,
  provider: ProvideAmountMethods,
  context: Chain,
}]
pub trait HasAmountMethods: HasAmountType + HasAsyncErrorType {
    fn add_amount(
        current: &Self::Amount,
        amount: &Self::Amount,
    ) -> Result<Self::Amount, Self::Error>;

    fn subtract_amount(
        current: &Self::Amount,
        amount: &Self::Amount,
    ) -> Result<Self::Amount, Self::Error>;
}
