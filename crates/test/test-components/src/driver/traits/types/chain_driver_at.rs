use core::marker::PhantomData;

use cgp::prelude::*;

#[cgp_type {
    name: ChainDriverTypeProviderAtComponent<I>,
    provider: ChainDriverTypeProviderAt,
}]
pub trait HasChainDriverTypeAt<I>: Async {
    type ChainDriver: Async;
}

pub type ChainDriverAt<Driver, I> = <Driver as HasChainDriverTypeAt<I>>::ChainDriver;

#[cgp_getter {
    name: ChainDriverGetterAtComponent<I>,
    provider: ChainDriverGetterAt,
}]
pub trait HasChainDriverAt<I>: HasChainDriverTypeAt<I> {
    fn chain_driver_at(&self, _tag: PhantomData<I>) -> &Self::ChainDriver;
}
