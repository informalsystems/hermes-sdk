use core::marker::PhantomData;

use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;

#[cgp_component {
  name: RelayDriverTypeAtComponent,
  provider: ProvideRelayDriverTypeAt,
  context: Driver,
}]
pub trait HasRelayDriverTypeAt<A: Async, B: Async>: HasBiRelayTypeAt<A, B> {
    type RelayDriver: HasBiRelayTypeAt<Index<0>, Index<1>, BiRelay = Self::BiRelay>;
}

#[cgp_component {
  provider: RelayDriverGetterAt,
  context: Driver,
}]
pub trait HasRelayDriverAt<A: Async, B: Async>: HasRelayDriverTypeAt<A, B> {
    fn relay_driver_at(&self, index: PhantomData<(A, B)>) -> &Self::RelayDriver;
}
