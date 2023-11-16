use core::time::Duration;

use alloc::boxed::Box;
use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_components::runtime::traits::sleep::CanSleep;

use crate::traits::chain::assert::eventual_amount::EventualAmountAsserter;
use crate::traits::chain::assert::poll_assert::HasPollAssertDuration;
use crate::traits::chain::queries::balance::CanQueryBalance;
use crate::traits::chain::types::address::HasAddressType;
use crate::traits::chain::types::amount::HasAmountType;

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
            let balance = chain.query_balance(address, denom).await?;

            if &balance == amount {
                return Ok(());
            } else {
                runtime.sleep(poll_interval).await;
            }
        }

        Err(chain.poll_assert_eventual_amount_timeout_error(
            address,
            amount,
            poll_interval * poll_attempts,
        ))
    }
}
