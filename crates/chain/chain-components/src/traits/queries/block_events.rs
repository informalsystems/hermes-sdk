use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::event::HasEventType;
use hermes_chain_type_components::traits::types::height::HasHeightType;

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
