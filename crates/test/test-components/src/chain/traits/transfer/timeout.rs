use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimeoutType;

#[cgp_component {
  provider: IbcTransferTimeoutCalculator,
  context: ChainDriver,
}]
pub trait CanCalculateIbcTransferTimeout: HasTimeoutType + HasHeightType {
    fn ibc_transfer_timeout_time(&self, current_time: &Self::Time) -> Option<Self::Timeout>;

    fn ibc_transfer_timeout_height(&self, current_height: &Self::Height) -> Option<Self::Height>;
}
