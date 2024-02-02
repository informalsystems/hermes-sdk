use core::time::Duration;

use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimestampType;
use hermes_test_components::chain_driver::traits::fields::timeout::IbcTransferTimeoutCalculator;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use ibc_relayer_types::timestamp::Timestamp;

pub struct IbcTransferTimeoutAfterSeconds<const SECS: u64>;

impl<ChainDriver, Chain, const SECS: u64> IbcTransferTimeoutCalculator<ChainDriver>
    for IbcTransferTimeoutAfterSeconds<SECS>
where
    ChainDriver: HasChainType<Chain = Chain>,
    Chain: HasTimestampType<Timestamp = Timestamp> + HasHeightType,
{
    fn ibc_transfer_timeout_time(
        _chain_driver: &ChainDriver,
        current_time: &Timestamp,
    ) -> Option<Timestamp> {
        let time = (*current_time + Duration::from_secs(SECS)).unwrap();
        Some(time)
    }

    fn ibc_transfer_timeout_height(
        _chain_driver: &ChainDriver,
        _current_height: &Chain::Height,
    ) -> Option<Chain::Height> {
        None
    }
}
