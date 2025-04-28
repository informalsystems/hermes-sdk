use core::marker::PhantomData;

use hermes_prelude::*;

#[cgp_type {
    name: RelayDriverTypeProviderAtComponent<A, B>,
    provider: RelayDriverTypeProviderAt,
}]
pub trait HasRelayDriverTypeAt<A, B>: Async {
    type RelayDriver: Async;
}

#[cgp_getter {
    name: RelayDriverGetterAtComponent<A, B>,
    provider: RelayDriverGetterAt,
}]
pub trait HasRelayDriverAt<A, B>: HasRelayDriverTypeAt<A, B> {
    fn relay_driver_at(&self, index: PhantomData<(A, B)>) -> &Self::RelayDriver;
}
