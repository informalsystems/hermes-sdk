use core::time::Duration;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::types::height::HasHeightFields;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;

use crate::chain_driver::traits::types::chain::HasChain;
use crate::chain_driver::traits::wait::ChainStartupWaiter;

pub struct WaitChainReachHeight<const H: u64>;

impl<ChainDriver, Chain, const H: u64> ChainStartupWaiter<ChainDriver> for WaitChainReachHeight<H>
where
    ChainDriver:
        HasChain<Chain = Chain> + HasRuntime<Runtime: CanSleep> + CanRaiseError<&'static str>,
    Chain: CanQueryChainHeight + HasHeightFields,
{
    async fn wait_chain_startup(chain_driver: &ChainDriver) -> Result<(), ChainDriver::Error> {
        let runtime = chain_driver.runtime();
        let chain = chain_driver.chain();

        for _ in 0..10 {
            if let Ok(height) = chain.query_chain_height().await {
                if Chain::revision_height(&height) >= H {
                    return Ok(());
                }
            }

            runtime.sleep(Duration::from_millis(500)).await;
        }

        Err(ChainDriver::raise_error(
            "chain did not progress to target height within 5 seconds",
        ))
    }
}

pub struct NoWaitChainStartup;

impl<ChainDriver> ChainStartupWaiter<ChainDriver> for NoWaitChainStartup
where
    ChainDriver: Async + HasErrorType,
{
    async fn wait_chain_startup(_chain_driver: &ChainDriver) -> Result<(), ChainDriver::Error> {
        Ok(())
    }
}
