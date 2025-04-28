use core::marker::PhantomData;
use core::time::Duration;

use hermes_prelude::*;

#[cgp_component {
    provider: BlockTimeQuerier,
    context: Chain,
}]
#[async_trait]
pub trait CanQueryBlockTime: HasAsyncErrorType {
    async fn query_block_time(&self) -> Result<Duration, Self::Error>;
}

#[cgp_provider(BlockTimeQuerierComponent)]
impl<Chain, Tag> BlockTimeQuerier<Chain> for UseField<Tag>
where
    Chain: HasField<Tag, Value = Duration> + HasAsyncErrorType,
{
    async fn query_block_time(chain: &Chain) -> Result<Duration, Chain::Error> {
        Ok(*chain.get_field(PhantomData))
    }
}
