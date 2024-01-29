use cgp_core::prelude::*;

use crate::birelay::traits::two_way::HasTwoWayRelayTypes;

/// Trait for types that have access to a bi-directional relayer
/// that can relay between two connected chains in both directions.
#[derive_component(BiRelayTypeComponent, ProvideBiRelayType<Context>)]
pub trait HasBiRelayType: HasErrorType {
    /// A relay context that can relay between two chains in a bi-
    /// directional fashion.
    type BiRelay: HasTwoWayRelayTypes;
}

pub trait HasBiRelay: HasBiRelayType {
    fn birelay(&self) -> &Self::BiRelay;
}

pub type BiRelayOf<Build> = <Build as HasBiRelayType>::BiRelay;
