use core::time::Duration;

use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimeoutType;
use hermes_test_components::chain::traits::transfer::timeout::IbcTransferTimeoutCalculator;
use ibc_relayer_types::timestamp::Timestamp;

pub struct IbcTransferTimeoutAfterSeconds<const SECS: u64>;

impl<Chain, const SECS: u64> IbcTransferTimeoutCalculator<Chain>
    for IbcTransferTimeoutAfterSeconds<SECS>
where
    Chain: HasTimeoutType<Timeout = Timestamp> + HasHeightType,
{
    fn ibc_transfer_timeout_time(_chain: &Chain, current_time: &Timestamp) -> Option<Timestamp> {
        let time = (*current_time + Duration::from_secs(SECS)).unwrap();
        Some(time)
    }

    fn ibc_transfer_timeout_height(
        _chain: &Chain,
        _current_height: &Chain::Height,
    ) -> Option<Chain::Height> {
        None
    }
}
