use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use crate::chain::traits::types::amount::HasAmountType;

#[cgp_component {
  provider: IbcTransferredAmountConverter,
  context: Chain,
}]
pub trait CanConvertIbcTransferredAmount<Counterparty>:
    HasAmountType + HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasAmountType,
{
    fn ibc_transfer_amount_from(
        counterparty_amount: &Counterparty::Amount,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
    ) -> Result<Self::Amount, Self::Error>;

    fn transmute_counterparty_amount(
        counterparty_amount: &Counterparty::Amount,
        denom: &Self::Denom,
    ) -> Self::Amount;
}
