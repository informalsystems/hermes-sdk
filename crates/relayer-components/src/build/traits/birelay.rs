use cgp_core::HasErrorType;

use crate::relay::traits::two_way::HasTwoWayRelayTypes;

/// Trait for types that have access to a bi-directional relayer
/// that can relay between two connected chains in both directions.
pub trait HasBiRelayType: HasErrorType {
    /// A relay context that can relay between two chains in a bi-
    /// directional fashion.
    type BiRelay: HasTwoWayRelayTypes;

    fn birelay_error(e: <Self::BiRelay as HasErrorType>::Error) -> Self::Error;
}

pub trait HasBiRelay: HasBiRelayType {
    fn birelay(&self) -> &Self::BiRelay;
}
