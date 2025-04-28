use alloc::vec::Vec;

use hermes_chain_type_components::traits::{HasEventType, HasHeightType};
use hermes_prelude::*;

#[cgp_component {
    provider: BlockEventsQuerier,
    context: Chain,
}]
#[async_trait]
pub trait CanQueryBlockEvents: HasHeightType + HasEventType + HasAsyncErrorType {
    async fn query_block_events(
        &self,
        height: &Self::Height,
    ) -> Result<Vec<Self::Event>, Self::Error>;
}
