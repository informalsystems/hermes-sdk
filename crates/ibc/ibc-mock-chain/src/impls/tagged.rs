use core::marker::PhantomData;

use cgp::core::types::traits::ProvideType;
use cgp::core::Async;

use crate::types::tagged::Tagged;

pub struct UseTaggedType<Provider>(pub PhantomData<Provider>);

impl<A: Async, B: Async, Value, Tag, Provider, Type> ProvideType<Tagged<A, B, Value>, Tag>
    for UseTaggedType<Provider>
where
    Provider: ProvideType<Tagged<A, B, Value>, Tag, Type = Type>,
{
    type Type = Tagged<A, B, Type>;
}
