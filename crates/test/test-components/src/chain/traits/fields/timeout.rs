use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimestampType;
use hermes_relayer_components::chain::types::aliases::{Height, Timestamp};

use crate::driver::traits::types::chain::HasChainType;

pub trait CanCalculateIbcTransferTimeout: HasChainType
where
    Self::Chain: HasTimestampType + HasHeightType,
{
    fn ibc_transfer_timeout_time(
        &self,
        current_time: &Timestamp<Self::Chain>,
    ) -> Option<Timestamp<Self::Chain>>;

    fn ibc_transfer_timeout_height(
        &self,
        current_height: &Height<Self::Chain>,
    ) -> Option<Height<Self::Chain>>;
}
