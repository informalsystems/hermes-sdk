/*!
   Trait definition for [`HasTimestampType`].
*/

use core::time::Duration;

pub use hermes_chain_type_components::traits::*;
use hermes_prelude::*;

#[cgp_component {
  provider: TimeMeasurer,
  context: Chain,
}]
pub trait CanMeasureTime: HasTimeType {
    fn duration_since(earlier: &Self::Time, later: &Self::Time) -> Option<Duration>;
}
