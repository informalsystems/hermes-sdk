use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::event::HasEventType;

use crate::sovereign::traits::rollup::types::event_id::HasEventIdType;

#[derive_component(EventsByEventIdsQuerierComponent, EventsByEventIdsQuerier<Rollup>)]
#[async_trait]
pub trait CanQueryEventsByEventIds: HasEventIdType + HasEventType + HasErrorType {
    async fn query_events_by_event_ids(
        &self,
        event_ids: &[Self::EventId],
    ) -> Result<Vec<Self::Event>, Self::Error>;
}
