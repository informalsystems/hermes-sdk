use core::time::Duration;

use hermes_chain_components::traits::HasHeightType;
use hermes_chain_components::types::aliases::HeightOf;
use hermes_prelude::*;

use crate::relay::traits::{HasTargetChainTypes, RelayTarget};

#[cgp_component {
    name: ClientRefresherComponent,
    provider: ClientRefresher,
    context: Relay,
}]
#[async_trait]
pub trait CanRefreshClient<Target>:
    HasTargetChainTypes<Target, TargetChain: HasHeightType> + HasAsyncErrorType
where
    Target: RelayTarget,
{
    async fn auto_refresh_client(
        &self,
        target: Target,
        interval: Duration,
        end_height: Option<&HeightOf<Self::TargetChain>>,
    ) -> Result<(), Self::Error>;
}
