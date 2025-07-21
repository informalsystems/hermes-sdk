use core::time::Duration;

use hermes_prelude::*;

use crate::relay::traits::{HasTargetChainTypes, RelayTarget};

#[cgp_component {
    name: ClientRefresherComponent,
    provider: ClientRefresher,
    context: Relay,
}]
#[async_trait]
pub trait CanRefreshClient<Target>: HasTargetChainTypes<Target> + HasAsyncErrorType
where
    Target: RelayTarget,
{
    async fn auto_refresh_client(
        &self,
        target: Target,
        interval: Duration,
    ) -> Result<(), Self::Error>;
}
