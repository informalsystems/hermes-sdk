use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::UseField;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

use crate::chain::types::aliases::ChainIdOf;

#[cgp_type {
    name: ChainTypeProviderAtComponent<I>,
    provider: ChainTypeProviderAt,
}]
pub trait HasChainTypeAt<I>: Async {
    type Chain: Async;
}

#[cgp_component {
    name: ChainGetterAtComponent<I>,
    provider: ChainGetterAt,
}]
pub trait HasChainAt<I>: HasChainTypeAt<I> {
    fn chain_at(&self, _tag: PhantomData<I>) -> &Self::Chain;
}

pub type ChainAt<Context, I> = <Context as HasChainTypeAt<I>>::Chain;

pub type ChainIdAt<Context, I> = ChainIdOf<ChainAt<Context, I>>;

#[cgp_provider(ChainTypeProviderAtComponent<I>)]
impl<Context, I, FieldTag, Chain> ChainTypeProviderAt<Context, I> for UseField<FieldTag>
where
    Context: Async + HasField<FieldTag, Value = Chain>,
    Chain: Async,
{
    type Chain = Chain;
}

#[cgp_provider(ChainGetterAtComponent<I>)]
impl<Context, I, FieldTag, Chain> ChainGetterAt<Context, I> for UseField<FieldTag>
where
    Context: HasChainTypeAt<I, Chain = Chain> + HasField<FieldTag, Value = Chain>,
{
    fn chain_at(context: &Context, _tag: PhantomData<I>) -> &Context::Chain {
        context.get_field(PhantomData)
    }
}
