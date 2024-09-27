/*!
   Trait definition for [`HasTimestampType`].
*/

use core::time::Duration;

use cgp::prelude::*;

#[derive_component(TimeTypeComponent, ProvideTimeType<Chain>)]
pub trait HasTimeType: Async {
    type Time: Async;

    fn duration_since(earlier: &Self::Time, later: &Self::Time) -> Option<Duration>;
}

#[derive_component(TimeoutTypeComponent, ProvideTimeoutType<Chain>)]
pub trait HasTimeoutType: HasTimeType {
    /**
       The timestamp of a chain, which should increment monotonically.

       By default, the timestamp only contains the `Ord` constraint, and does
       not support operations like adding to a `Duration`.

       We can impose additional constraints at the use site of `HasChainTypes`.
       However doing so may impose limitations on which concrete types
       the `Timestamp` type can be.

       By keeping the abstract type minimal, we can for example use
       simple `u8` or `u128` in seconds as the `Timestamp` type during testing,
       and use the more complex types like `DateTime` type during production.

       This especially helps given that having a canonical time type is
       still largely an unsolved problem in software engineering. Depending
       on the specific use cases, different concrete contexts may want to
       use different date time types to enforce certain invariants.
       By keeping this type abstract, we provide the flexibility to
       concrete context implementers to decide which exact time type
       they would like to use.
    */
    type Timeout: Async;

    fn has_timed_out(time: &Self::Time, timeout: &Self::Timeout) -> bool;
}
