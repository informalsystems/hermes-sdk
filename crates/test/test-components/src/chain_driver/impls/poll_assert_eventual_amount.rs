use core::fmt::Debug;
use core::time::Duration;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_relayer_components::runtime::traits::sleep::CanSleep;

use crate::chain::traits::queries::balance::CanQueryBalance;
use crate::chain::traits::types::address::AddressOf;
use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::AmountOf;
use crate::chain::traits::types::amount::HasAmountType;
use crate::chain_driver::traits::assert::eventual_amount::EventualAmountAsserter;
use crate::chain_driver::traits::assert::poll_assert::HasPollAssertDuration;
use crate::chain_driver::traits::types::chain::HasChain;
use crate::chain_driver::traits::types::chain::HasChainType;

pub struct PollAssertEventualAmount;

#[async_trait]
impl<ChainDriver, Chain> EventualAmountAsserter<ChainDriver> for PollAssertEventualAmount
where
    ChainDriver: HasRuntime
        + HasChain<Chain = Chain>
        + HasPollAssertDuration
        + for<'a> CanRaiseError<EventualAmountTimeoutError<'a, ChainDriver>>,
    Chain: CanQueryBalance,
    ChainDriver::Runtime: CanSleep,
{
    async fn assert_eventual_amount(
        chain_driver: &ChainDriver,
        address: &Chain::Address,
        amount: &Chain::Amount,
    ) -> Result<(), ChainDriver::Error> {
        let poll_interval = chain_driver.poll_assert_interval();
        let poll_attempts = chain_driver.poll_assert_attempts();

        let denom = Chain::amount_denom(amount);
        let runtime = chain_driver.runtime();
        let chain = chain_driver.chain();

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
    ChainDriver: HasChainType + HasErrorType,
    ChainDriver::Chain: HasAddressType + HasAmountType,
{
    pub chain_driver: &'a ChainDriver,
    pub address: &'a AddressOf<ChainDriver::Chain>,
    pub amount: &'a AmountOf<ChainDriver::Chain>,
    pub duration: Duration,
}

impl<'a, ChainDriver, Chain> Debug for EventualAmountTimeoutError<'a, ChainDriver>
where
    ChainDriver: HasChainType<Chain = Chain> + HasErrorType,
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
