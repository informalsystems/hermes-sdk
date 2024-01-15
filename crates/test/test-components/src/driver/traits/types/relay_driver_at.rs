use hermes_relayer_components::build::traits::birelay::HasBiRelayType;

use crate::driver::traits::types::birelay_at::HasBiRelayTypeAt;
use crate::types::index::Twindex;

pub trait HasRelayDriverTypeAt<const A: usize, const B: usize>: HasBiRelayTypeAt<A, B> {
    type RelayDriver: HasBiRelayType<BiRelay = Self::BiRelay>;
}

pub trait HasRelayDriverAt<const A: usize, const B: usize>: HasRelayDriverTypeAt<A, B> {
    fn relay_driver_at(&self, index: Twindex<A, B>) -> &Self::RelayDriver;
}
