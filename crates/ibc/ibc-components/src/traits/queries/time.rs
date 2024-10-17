use cgp::prelude::*;

use hermes_chain_type_components::traits::types::time::HasTimeType;

#[derive_component(CurrentTimeQuerierComponent, CurrentTimeQuerier<Chain>)]
#[async_trait]
pub trait CanQueryCurrentTime: HasTimeType {
    async fn get_current_time(&self) -> Self::Time;
}
