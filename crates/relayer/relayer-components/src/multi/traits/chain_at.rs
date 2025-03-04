use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::UseField;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

use crate::chain::types::aliases::ChainIdOf;

#[cgp_component {
  name: ChainTypeAtComponent<Tag>,
  provider: ProvideChainTypeAt,
}]
pub trait HasChainTypeAt<Tag>: Async {
    type Chain: Async;
}

#[cgp_component {
  name: ChainGetterAtComponent<Tag>,
  provider: ChainGetterAt,
}]
pub trait HasChainAt<Tag>: HasChainTypeAt<Tag> {
    fn chain_at(&self, _tag: PhantomData<Tag>) -> &Self::Chain;
}

pub type ChainAt<Context, Tag> = <Context as HasChainTypeAt<Tag>>::Chain;

pub type ChainIdAt<Context, Tag> = ChainIdOf<ChainAt<Context, Tag>>;

#[cgp_provider(ChainTypeAtComponent<Tag>)]
impl<Context, Tag, Provider, Chain> ProvideChainTypeAt<Context, Tag> for WithProvider<Provider>
where
    Provider: ProvideType<Context, ChainTypeAtComponent<Tag>, Type = Chain>,
    Context: Async,
    Chain: Async,
{
    type Chain = Chain;
}

#[cgp_provider(ChainTypeAtComponent<ChainTag>)]
impl<Context, ChainTag, FieldTag, Chain> ProvideChainTypeAt<Context, ChainTag>
    for UseField<FieldTag>
where
    Context: Async + HasField<FieldTag, Value = Chain>,
    Chain: Async,
{
    type Chain = Chain;
}

#[cgp_provider(ChainGetterAtComponent<ChainTag>)]
impl<Context, ChainTag, FieldTag, Chain> ChainGetterAt<Context, ChainTag> for UseField<FieldTag>
where
    Context: HasChainTypeAt<ChainTag, Chain = Chain> + HasField<FieldTag, Value = Chain>,
{
    fn chain_at(context: &Context, _tag: PhantomData<ChainTag>) -> &Context::Chain {
        context.get_field(PhantomData)
    }
}
