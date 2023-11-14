use cgp_core::HasErrorType;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use crate::traits::chain::types::amount::HasAmountType;
use crate::traits::chain::types::denom::HasDenomType;

pub trait CanGenerateRandomAmount: HasDenomType + HasAmountType {
    fn random_amount(min: usize, max: &Self::Amount) -> Self::Amount;
}

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

pub trait HasIbcTransferredAmount<Counterparty>:
    HasAmountType + HasIbcChainTypes<Counterparty>
where
    Counterparty: HasAmountType,
{
    fn ibc_transfer_amount_from(
        counterparty_amount: &Counterparty::Amount,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
    ) -> Self::Amount;
}
