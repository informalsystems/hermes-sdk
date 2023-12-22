use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::denom::HasDenomType;

#[derive_component(RandomAmountGeneratorComponent, RandomAmountGenerator<Chain>)]
pub trait CanGenerateRandomAmount: HasDenomType + HasAmountType {
    fn random_amount(min: usize, max: &Self::Amount) -> Self::Amount;
}

#[derive_component(AmountMethodsProviderComponent, AmountMethodsProvider<Chain>)]
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
pub trait CanConvertIbcTransferredAmount<Counterparty>:
    HasAmountType + HasIbcChainTypes<Counterparty>
where
    Counterparty: HasAmountType,
{
    fn ibc_transfer_amount_from(
        counterparty_amount: &Counterparty::Amount,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
    ) -> Self::Amount;

    fn transmute_counterparty_amount(
        counterparty_amount: &Counterparty::Amount,
        denom: &Self::Denom,
    ) -> Self::Amount;
}
