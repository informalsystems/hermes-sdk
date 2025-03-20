use core::marker::PhantomData;

use cgp::prelude::*;

#[cgp_component {
    name: BootstrapAtComponent<I>,
    provider: ProvideBootstrapAt,
    context: Setup,
}]
pub trait HasBootstrapAt<I>: Async {
    type Bootstrap: Async;

    fn chain_bootstrap(&self, _tag: PhantomData<I>) -> &Self::Bootstrap;
}

pub type BootstrapAt<Context, Tag> = <Context as HasBootstrapAt<Tag>>::Bootstrap;
