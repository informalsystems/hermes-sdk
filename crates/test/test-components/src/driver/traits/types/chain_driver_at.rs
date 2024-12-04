use core::marker::PhantomData;

use cgp::prelude::*;

#[derive_component(ChainDriverTypeAtComponent, ProvideChainDriverTypeAt<Driver>)]
pub trait HasChainDriverTypeAt<Tag>: Async {
    type ChainDriver: Async;
}

pub type ChainDriverTypeAt<Driver, Tag> = <Driver as HasChainDriverTypeAt<Tag>>::ChainDriver;

#[derive_component(ChainDriverGetterAtComponent, ChainDriverGetterAt<Driver>)]
pub trait HasChainDriverAt<Tag>: HasChainDriverTypeAt<Tag> {
    fn chain_driver_at(&self, _tag: PhantomData<Tag>) -> &Self::ChainDriver;
}
