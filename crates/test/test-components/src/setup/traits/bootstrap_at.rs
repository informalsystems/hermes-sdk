use core::marker::PhantomData;

use hermes_prelude::*;

#[cgp_type {
    name: BootstrapTypeProviderAtComponent<I>,
    provider: BootstrapTypeProviderAt,
}]
pub trait HasBootstrapTypeAt<I>: Async {
    type Bootstrap: Async;
}

#[cgp_getter {
    name: BootstrapGetterAtComponent<I>,
    provider: BootstrapGetterAt,
}]
pub trait HasBootstrapAt<I>: HasBootstrapTypeAt<I> {
    fn chain_bootstrap(&self, _tag: PhantomData<I>) -> &Self::Bootstrap;
}

pub type BootstrapAt<Context, Tag> = <Context as HasBootstrapTypeAt<Tag>>::Bootstrap;
