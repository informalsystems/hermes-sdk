use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;
use hermes_relayer_components::multi::types::index::Index;

#[derive_component(RelayDriverTypeAtComponent, ProvideRelayDriverTypeAt<Driver>)]
pub trait HasRelayDriverTypeAt<A: Async, B: Async>: HasBiRelayTypeAt<A, B> {
    type RelayDriver: HasBiRelayTypeAt<Index<0>, Index<1>, BiRelay = Self::BiRelay>;
}

#[derive_component(RelayDriverGetterAtComponent, RelayDriverGetterAt<Driver>)]
pub trait HasRelayDriverAt<A: Async, B: Async>: HasRelayDriverTypeAt<A, B> {
    fn relay_driver_at(&self, index: PhantomData<(A, B)>) -> &Self::RelayDriver;
}
