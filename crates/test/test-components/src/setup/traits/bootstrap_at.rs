use core::marker::PhantomData;

use cgp::prelude::*;

#[cgp_type {
    name: BootstrapTypeProviderAtComponent<I>,
    provider: BootstrapTypeProviderAt,
}]
pub trait HasBootstrapTypeAt<I>: Async {
    type Bootstrap: Async;
}

#[cgp_component {
    name: BootstrapGetterAtComponent<I>,
    provider: BootstrapGetterAt,
}]
pub trait HasBootstrapAt<I>: HasBootstrapTypeAt<I> {
    fn chain_bootstrap(&self, _tag: PhantomData<I>) -> &Self::Bootstrap;
}

pub type BootstrapAt<Context, Tag> = <Context as HasBootstrapTypeAt<Tag>>::Bootstrap;

#[cgp_provider(BootstrapGetterAtComponent<I>)]
impl<Setup, I, Tag> BootstrapGetterAt<Setup, I> for UseField<Tag>
where
    Setup: HasBootstrapTypeAt<I> + HasField<Tag, Value = Setup::Bootstrap>,
{
    fn chain_bootstrap(setup: &Setup, _tag: PhantomData<I>) -> &Setup::Bootstrap {
        setup.get_field(PhantomData)
    }
}
