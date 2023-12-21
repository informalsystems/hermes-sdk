use alloc::boxed::Box;
use core::time::Duration;

use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_relayer_components::runtime::traits::sleep::CanSleep;

use crate::chain::traits::assert::eventual_amount::EventualAmountAsserter;
use crate::chain::traits::assert::poll_assert::HasPollAssertDuration;
use crate::chain::traits::queries::balance::CanQueryBalance;
use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;

pub trait CanRaisePollAssertEventualAmountTimeout:
    HasAddressType + HasAmountType + HasErrorType
{
    fn poll_assert_eventual_amount_timeout_error(
        &self,
        address: &Self::Address,
        amount: &Self::Amount,
        duration: Duration,
    ) -> Self::Error;
}

pub struct PollAssertEventualAmount;

#[async_trait]
impl<Chain> EventualAmountAsserter<Chain> for PollAssertEventualAmount
where
    Chain: HasRuntime
        + CanQueryBalance
        + HasPollAssertDuration
        + CanRaisePollAssertEventualAmountTimeout,
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

        Err(chain.poll_assert_eventual_amount_timeout_error(
            address,
            amount,
            poll_interval * poll_attempts,
        ))
    }
}
