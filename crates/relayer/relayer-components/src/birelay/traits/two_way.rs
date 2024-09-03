use cgp::prelude::*;

use crate::multi::traits::relay_at::{HasRelayTypeAt, RelayAt};

/// Trait for types that have a two-way relay context, i.e.,
/// those that can relay in both directions between two connected
/// chains.
///
/// Two-way relay contexts are composed of two separate relay
/// contexts, one that relays from chain A to chain B, the
/// other that relays from chain B to chain A.
pub trait HasTwoWayRelayTypes: HasRelayTypeAt<0, 1> + HasRelayTypeAt<1, 0> {}

impl<BiRelay> HasTwoWayRelayTypes for BiRelay where
    BiRelay: HasRelayTypeAt<0, 1> + HasRelayTypeAt<1, 0>
{
}

#[derive_component(TwoWayRelayGetterComponent, TwoWayRelayGetter<BiRelay>)]
pub trait HasTwoWayRelay: HasTwoWayRelayTypes {
    /// Returns a read-only reference to the relay context from chain A
    /// to chain B.
    fn relay_a_to_b(&self) -> &RelayAt<Self, 0, 1>;

    /// Returns a read-only reference to the relay context from chain B
    /// to chain A.
    fn relay_b_to_a(&self) -> &RelayAt<Self, 1, 0>;
}
