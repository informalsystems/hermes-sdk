use cgp_core::{CanRaiseError, HasErrorType};

use crate::birelay::traits::two_way::HasTwoWayRelayTypes;

/// Trait for types that have access to a bi-directional relayer
/// that can relay between two connected chains in both directions.
pub trait HasBiRelayType:
    HasErrorType + CanRaiseError<<Self::BiRelay as HasErrorType>::Error>
{
    /// A relay context that can relay between two chains in a bi-
    /// directional fashion.
    type BiRelay: HasTwoWayRelayTypes;
}

pub trait HasBiRelay: HasBiRelayType {
    fn birelay(&self) -> &Self::BiRelay;
}
