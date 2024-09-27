/*!
   Trait definition for [`HasTimestampType`].
*/

use core::time::Duration;

use cgp::prelude::*;

pub use hermes_chain_type_components::traits::types::time::*;
pub use hermes_chain_type_components::traits::types::timeout::*;

#[derive_component(TimeMeasurerComponent, TimeMeasurer<Chain>)]
pub trait CanMeasureTime: HasTimeType {
    fn duration_since(earlier: &Self::Time, later: &Self::Time) -> Option<Duration>;
}
