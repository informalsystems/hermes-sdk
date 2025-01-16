use core::marker::PhantomData;

use cgp::core::types::ProvideType;
use cgp::core::Async;

use crate::contexts::chain::MockChain;
use crate::types::tagged::Tagged;

pub struct UseTaggedType<Provider>(pub PhantomData<Provider>);

impl<A: Async, B: Async, Tag, Provider, Type> ProvideType<MockChain<A, B>, Tag>
    for UseTaggedType<Provider>
where
    Provider: ProvideType<MockChain<A, B>, Tag, Type = Type>,
{
    type Type = Tagged<A, B, Type>;
}
