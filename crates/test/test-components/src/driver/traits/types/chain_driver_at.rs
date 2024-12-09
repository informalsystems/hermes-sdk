use core::marker::PhantomData;

use cgp::prelude::*;

#[cgp_component {
  name: ChainDriverTypeAtComponent,
  provider: ProvideChainDriverTypeAt,
  context: Driver,
}]
pub trait HasChainDriverTypeAt<Tag>: Async {
    type ChainDriver: Async;
}

pub type ChainDriverTypeAt<Driver, Tag> = <Driver as HasChainDriverTypeAt<Tag>>::ChainDriver;

#[cgp_component {
  provider: ChainDriverGetterAt,
  context: Driver,
}]
pub trait HasChainDriverAt<Tag>: HasChainDriverTypeAt<Tag> {
    fn chain_driver_at(&self, _tag: PhantomData<Tag>) -> &Self::ChainDriver;
}
