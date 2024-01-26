use core::fmt::Debug;
use core::time::Duration;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_relayer_components::runtime::traits::sleep::CanSleep;

use crate::chain_driver::traits::assert::eventual_amount::EventualAmountAsserter;
use crate::chain_driver::traits::assert::poll_assert::HasPollAssertDuration;
use crate::chain_driver::traits::queries::balance::CanQueryBalance;
use crate::chain_driver::traits::types::address::HasAddressType;
use crate::chain_driver::traits::types::amount::HasAmountType;

pub struct PollAssertEventualAmount;

#[async_trait]
impl<ChainDriver> EventualAmountAsserter<ChainDriver> for PollAssertEventualAmount
where
    ChainDriver: HasRuntime
        + CanQueryBalance
        + HasPollAssertDuration
        + for<'a> CanRaiseError<EventualAmountTimeoutError<'a, ChainDriver>>,
    ChainDriver::Runtime: CanSleep,
{
    async fn assert_eventual_amount(
        chain_driver: &ChainDriver,
        address: &ChainDriver::Address,
        amount: &ChainDriver::Amount,
    ) -> Result<(), ChainDriver::Error> {
        let poll_interval = chain_driver.poll_assert_interval();
        let poll_attempts = chain_driver.poll_assert_attempts();

        let denom = ChainDriver::amount_denom(amount);
        let runtime = chain_driver.runtime();

        for _ in 0..poll_attempts {
            let balance_result = chain_driver.query_balance(address, denom).await;

            match balance_result {
                Ok(balance) if &balance == amount => {
                    return Ok(());
                }
                _ => {
                    runtime.sleep(poll_interval).await;
                }
            };
        }

        Err(ChainDriver::raise_error(EventualAmountTimeoutError {
            chain_driver,
            address,
            amount,
            duration: poll_interval * poll_attempts,
        }))
    }
}

pub struct EventualAmountTimeoutError<'a, ChainDriver>
where
    ChainDriver: HasAddressType + HasAmountType + HasErrorType,
{
    pub chain_driver: &'a ChainDriver,
    pub address: &'a ChainDriver::Address,
    pub amount: &'a ChainDriver::Amount,
    pub duration: Duration,
}

impl<'a, ChainDriver> Debug for EventualAmountTimeoutError<'a, ChainDriver>
where
    ChainDriver: HasAddressType + HasAmountType + HasErrorType,
    ChainDriver::Address: Debug,
    ChainDriver::Amount: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EventualAmountTimeoutError")
            .field(
                "message",
                &"timeout waiting for the balance of address to reach the expected amount",
            )
            .field("address", &self.address)
            .field("amount", &self.amount)
            .field("duration", &self.duration)
            .finish()
    }
}
