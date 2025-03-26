use core::marker::PhantomData;

use cgp::core::field::Index;
use cgp::core::macros::blanket_trait;

use crate::multi::traits::relay_at::{HasRelayAt, HasRelayTypeAt, RelayAt};

/// Trait for types that have a two-way relay context, i.e.,
/// those that can relay in both directions between two connected
/// chains.
///
/// Two-way relay contexts are composed of two separate relay
/// contexts, one that relays from chain A to chain B, the
/// other that relays from chain B to chain A.
#[blanket_trait]
pub trait HasTwoWayRelayTypes:
    HasRelayTypeAt<Index<0>, Index<1>> + HasRelayTypeAt<Index<1>, Index<0>>
{
}

#[blanket_trait]
pub trait HasTwoWayRelay: HasRelayAt<Index<0>, Index<1>> + HasRelayAt<Index<1>, Index<0>> {
    /// Returns a read-only reference to the relay context from chain A
    /// to chain B.
    fn relay_a_to_b(&self) -> &RelayAt<Self, Index<0>, Index<1>> {
        self.relay_at(PhantomData::<(Index<0>, Index<1>)>)
    }

    /// Returns a read-only reference to the relay context from chain B
    /// to chain A.
    fn relay_b_to_a(&self) -> &RelayAt<Self, Index<1>, Index<0>> {
        self.relay_at(PhantomData::<(Index<1>, Index<0>)>)
    }
}
