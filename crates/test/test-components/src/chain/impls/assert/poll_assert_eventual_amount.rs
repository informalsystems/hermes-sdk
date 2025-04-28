use alloc::format;
use core::fmt::Debug;
use core::time::Duration;

use hermes_chain_type_components::traits::{HasAddressType, HasAmountDenom, HasAmountType};
use hermes_logging_components::traits::CanLog;
use hermes_logging_components::types::LevelError;
use hermes_prelude::*;
use hermes_runtime_components::traits::{CanSleep, HasRuntime};

use crate::chain::traits::{
    CanQueryBalance, EventualAmountAsserter, EventualAmountAsserterComponent, HasPollAssertDuration,
};

#[cgp_new_provider(EventualAmountAsserterComponent)]
impl<Chain> EventualAmountAsserter<Chain> for PollAssertEventualAmount
where
    Chain: HasRuntime
        + HasPollAssertDuration
        + CanQueryBalance
        + HasAmountDenom
        + CanLog<LevelError>
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

        let mut balance_result = chain.query_balance(address, denom).await;

        for _ in 1..poll_attempts {
            match balance_result {
                Ok(balance) if &balance == amount => {
                    return Ok(());
                }
                _ => {
                    runtime.sleep(poll_interval).await;
                }
            };
            balance_result = chain.query_balance(address, denom).await;
        }

        let final_balance = balance_result?;

        chain
            .log(
                &format!("Expected balance `{amount}`, found `{final_balance}`"),
                &LevelError,
            )
            .await;

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
