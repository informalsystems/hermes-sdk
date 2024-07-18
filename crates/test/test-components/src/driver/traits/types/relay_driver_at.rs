use cgp_core::prelude::*;
use hermes_relayer_components::build::traits::birelay::HasBiRelayType;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;
use hermes_relayer_components::multi::types::index::Twindex;

#[derive_component(RelayDriverTypeAtComponent, ProvideRelayDriverTypeAt<Driver>)]
pub trait HasRelayDriverTypeAt<const A: usize, const B: usize>: HasBiRelayTypeAt<A, B> {
    type RelayDriver: HasBiRelayType<BiRelay = Self::BiRelay>;
}

#[derive_component(RelayDriverGetterAtComponent, RelayDriverGetterAt<Driver>)]
pub trait HasRelayDriverAt<const A: usize, const B: usize>: HasRelayDriverTypeAt<A, B> {
    fn relay_driver_at(&self, index: Twindex<A, B>) -> &Self::RelayDriver;
}
