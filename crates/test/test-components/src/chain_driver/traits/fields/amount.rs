use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, PortIdOf};

use crate::chain_driver::traits::types::amount::HasAmountType;
use crate::chain_driver::traits::types::chain::HasChainType;
use crate::chain_driver::traits::types::denom::HasDenomType;

#[derive_component(RandomAmountGeneratorComponent, RandomAmountGenerator<Chain>)]
pub trait CanGenerateRandomAmount: HasDenomType + HasAmountType {
    fn random_amount(min: usize, max: &Self::Amount) -> Self::Amount;
}

#[derive_component(AmountMethodsComponent, ProvideAmountMethods<Chain>)]
pub trait HasAmountMethods: HasAmountType + HasErrorType {
    fn add_amount(
        current: &Self::Amount,
        amount: &Self::Amount,
    ) -> Result<Self::Amount, Self::Error>;

    fn subtract_amount(
        current: &Self::Amount,
        amount: &Self::Amount,
    ) -> Result<Self::Amount, Self::Error>;
}

#[derive_component(IbcTransferredAmountConverterComponent, IbcTransferredAmountConverter<Chain>)]
pub trait CanConvertIbcTransferredAmount<CounterpartyDriver>:
    HasAmountType + HasChainType + HasErrorType
where
    Self::Chain: HasIbcChainTypes<CounterpartyDriver::Chain>,
    CounterpartyDriver: HasChainType + HasAmountType,
{
    fn ibc_transfer_amount_from(
        counterparty_amount: &CounterpartyDriver::Amount,
        channel_id: &ChannelIdOf<Self::Chain, CounterpartyDriver::Chain>,
        port_id: &PortIdOf<Self::Chain, CounterpartyDriver::Chain>,
    ) -> Result<Self::Amount, Self::Error>;

    fn transmute_counterparty_amount(
        counterparty_amount: &CounterpartyDriver::Amount,
        denom: &Self::Denom,
    ) -> Self::Amount;
}
