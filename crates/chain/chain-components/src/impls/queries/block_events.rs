use alloc::vec::Vec;
use core::marker::PhantomData;
use core::time::Duration;

use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasEventType, HasHeightType};
use hermes_runtime_components::traits::{CanSleep, HasRuntime};

use crate::traits::{
    BlockEventsQuerier, BlockEventsQuerierComponent, CanQueryChainHeight, HasPollInterval,
};

pub struct WaitBlockHeightAndQueryEvents<InQuerier>(pub PhantomData<InQuerier>);

#[cgp_provider(BlockEventsQuerierComponent)]
impl<Chain, InQuerier> BlockEventsQuerier<Chain> for WaitBlockHeightAndQueryEvents<InQuerier>
where
    Chain: HasRuntime + HasEventType + CanQueryChainHeight + HasPollInterval,
    InQuerier: BlockEventsQuerier<Chain>,
    Chain::Runtime: CanSleep,
{
    async fn query_block_events(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Vec<Chain::Event>, Chain::Error> {
        let runtime = chain.runtime();

        loop {
            let current_height = chain.query_chain_height().await?;
            if &current_height >= height {
                break;
            } else {
                runtime.sleep(chain.poll_interval()).await;
            }
        }

        InQuerier::query_block_events(chain, height).await
    }
}

pub struct RetryQueryBlockEvents<const MAX_RETRIES: usize, InQuerier>(pub PhantomData<InQuerier>);

#[cgp_provider(BlockEventsQuerierComponent)]
impl<Chain, InQuerier, const MAX_RETRIES: usize> BlockEventsQuerier<Chain>
    for RetryQueryBlockEvents<MAX_RETRIES, InQuerier>
where
    Chain: HasRuntime + HasHeightType + HasEventType + HasAsyncErrorType,
    InQuerier: BlockEventsQuerier<Chain>,
    Chain::Runtime: CanSleep,
{
    async fn query_block_events(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Vec<Chain::Event>, Chain::Error> {
        let runtime = chain.runtime();
        let mut sleep_time = Duration::from_millis(500);

        for _ in 0..MAX_RETRIES {
            let res = InQuerier::query_block_events(chain, height).await;
            if let Ok(events) = res {
                return Ok(events);
            }

            runtime.sleep(sleep_time).await;
            sleep_time *= 2;
        }

        InQuerier::query_block_events(chain, height).await
    }
}
