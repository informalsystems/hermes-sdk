use core::time::Duration;

use hermes_core::relayer_components::chain::traits::{HasHeightType, HasTimeType, HasTimeoutType};
use hermes_core::test_components::chain::traits::{
    IbcTransferTimeoutCalculator, IbcTransferTimeoutCalculatorComponent,
};
use hermes_cosmos_chain_components::types::Time;
use hermes_prelude::*;
use ibc::primitives::Timestamp;
use time::OffsetDateTime;

pub struct IbcTransferTimeoutAfterSeconds<const SECS: u64>;

#[cgp_provider(IbcTransferTimeoutCalculatorComponent)]
impl<Chain, Counterparty, const SECS: u64> IbcTransferTimeoutCalculator<Chain, Counterparty>
    for IbcTransferTimeoutAfterSeconds<SECS>
where
    Counterparty: HasTimeType<Time = Time> + HasTimeoutType<Timeout = Timestamp> + HasHeightType,
{
    fn ibc_transfer_timeout_time(_chain: &Chain, current_time: &Time) -> Option<Timestamp> {
        let time = (*current_time + Duration::from_secs(SECS)).unwrap();
        OffsetDateTime::from(time).try_into().ok()
    }

    fn ibc_transfer_timeout_height(
        _chain: &Chain,
        _current_height: &Counterparty::Height,
    ) -> Option<Counterparty::Height> {
        None
    }
}
