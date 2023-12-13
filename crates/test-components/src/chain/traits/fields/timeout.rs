use ibc_relayer_components::chain::traits::types::height::HasHeightType;
use ibc_relayer_components::chain::traits::types::timestamp::HasTimestampType;

pub trait CanCalculateIbcTransferTimeout: HasTimestampType + HasHeightType {
    fn ibc_transfer_timeout_time(&self, current_time: &Self::Timestamp) -> Option<Self::Timestamp>;

    fn ibc_transfer_timeout_height(&self, current_height: &Self::Height) -> Option<Self::Height>;
}
