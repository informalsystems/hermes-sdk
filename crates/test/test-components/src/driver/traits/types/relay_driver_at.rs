use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;

#[cgp_type {
    name: RelayDriverTypeProviderAtComponent<A, B>,
    provider: RelayDriverTypeProviderAt,
}]
pub trait HasRelayDriverTypeAt<A, B>: HasBiRelayTypeAt<A, B> {
    type RelayDriver: Async;
}

#[cgp_getter {
    name: RelayDriverGetterAtComponent<A, B>,
    provider: RelayDriverGetterAt,
}]
pub trait HasRelayDriverAt<A, B>: HasRelayDriverTypeAt<A, B> {
    fn relay_driver_at(&self, index: PhantomData<(A, B)>) -> &Self::RelayDriver;
}
