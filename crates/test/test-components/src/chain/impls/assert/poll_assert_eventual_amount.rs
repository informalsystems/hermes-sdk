use core::fmt::Debug;
use core::time::Duration;

use cgp::core::error::CanRaiseAsyncError;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;

use crate::chain::traits::assert::eventual_amount::EventualAmountAsserter;
use crate::chain::traits::assert::poll_assert::HasPollAssertDuration;
use crate::chain::traits::queries::balance::CanQueryBalance;
use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;

pub struct PollAssertEventualAmount;

impl<Chain> EventualAmountAsserter<Chain> for PollAssertEventualAmount
where
    Chain: HasRuntime
        + HasPollAssertDuration
        + CanQueryBalance
        + for<'a> CanRaiseAsyncError<EventualAmountTimeoutError<'a, Chain>>,
    Chain::Runtime: CanSleep,
{
    async fn assert_eventual_amount(
        chain: &Chain,
        address: &Chain::Address,
        amount: &Chain::Amount,
    ) -> Result<(), Chain::Error> {
        let poll_interval = chain.poll_assert_interval();
        let poll_attempts = chain.poll_assert_attempts();

        let denom = Chain::amount_denom(amount);
        let runtime = chain.runtime();

        for _ in 0..poll_attempts {
            let balance_result = chain.query_balance(address, denom).await;

            match balance_result {
                Ok(balance) if &balance == amount => {
                    return Ok(());
                }
                _ => {
                    runtime.sleep(poll_interval).await;
                }
            };
        }

        Err(Chain::raise_error(EventualAmountTimeoutError {
            chain,
            address,
            amount,
            duration: poll_interval * poll_attempts,
        }))
    }
}

pub struct EventualAmountTimeoutError<'a, Chain>
where
    Chain: HasAddressType + HasAmountType,
{
    pub chain: &'a Chain,
    pub address: &'a Chain::Address,
    pub amount: &'a Chain::Amount,
    pub duration: Duration,
}

impl<Chain> Debug for EventualAmountTimeoutError<'_, Chain>
where
    Chain: HasAddressType + HasAmountType,
    Chain::Address: Debug,
    Chain::Amount: Debug,
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
