use core::marker::PhantomData;

use cgp::prelude::*;

use crate::chain::types::aliases::ChainIdOf;

#[derive_component(ChainTypeAtComponent, ProvideChainTypeAt<Context>)]
pub trait HasChainTypeAt<Tag>: Async {
    type Chain: Async;
}

pub trait HasChainAt<Tag>: HasChainTypeAt<Tag> {
    fn chain_at(&self, _tag: PhantomData<Tag>) -> &Self::Chain;
}

pub type ChainAt<Context, Tag> = <Context as HasChainTypeAt<Tag>>::Chain;

pub type ChainIdAt<Context, Tag> = ChainIdOf<ChainAt<Context, Tag>>;
