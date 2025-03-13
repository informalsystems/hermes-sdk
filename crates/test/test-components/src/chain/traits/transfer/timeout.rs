use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimeoutType;

#[cgp_component {
  provider: IbcTransferTimeoutCalculator,
  context: Chain,
}]
pub trait CanCalculateIbcTransferTimeout<Counterparty>
where
    Counterparty: HasTimeoutType + HasHeightType,
{
    fn ibc_transfer_timeout_time(
        &self,
        current_time: &Counterparty::Time,
    ) -> Option<Counterparty::Timeout>;

    fn ibc_transfer_timeout_height(
        &self,
        current_height: &Counterparty::Height,
    ) -> Option<Counterparty::Height>;
}
