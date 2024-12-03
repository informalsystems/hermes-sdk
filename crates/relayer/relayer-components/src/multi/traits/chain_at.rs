use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

use crate::chain::types::aliases::ChainIdOf;

#[derive_component(ChainTypeAtComponent<Tag>, ProvideChainTypeAt<Context>)]
pub trait HasChainTypeAt<Tag>: Async {
    type Chain: Async;
}

pub trait HasChainAt<Tag>: HasChainTypeAt<Tag> {
    fn chain_at(&self, _tag: PhantomData<Tag>) -> &Self::Chain;
}

pub type ChainAt<Context, Tag> = <Context as HasChainTypeAt<Tag>>::Chain;

pub type ChainIdAt<Context, Tag> = ChainIdOf<ChainAt<Context, Tag>>;

impl<Context, Tag, Provider, Chain> ProvideChainTypeAt<Context, Tag> for WithProvider<Provider>
where
    Provider: ProvideType<Context, ChainTypeAtComponent<Tag>, Type = Chain>,
    Context: Async,
    Chain: Async,
{
    type Chain = Chain;
}
